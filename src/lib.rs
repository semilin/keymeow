use kc::{
    self,
    analysis::{Analyzer, MetricData as KcMetricData, NstrokeData},
    Corpus, CorpusChar, Layout, NgramType,
};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

pub enum Hand {
    Left,
    Right,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Finger {
    LP,
    LR,
    LM,
    LI,
    LT,
    RT,
    RI,
    RM,
    RR,
    RP,
}

impl Finger {
    #[allow(dead_code)]
    const LIST: [Self; 10] = [
        Finger::LP,
        Finger::LR,
        Finger::LM,
        Finger::LI,
        Finger::LT,
        Finger::RT,
        Finger::RI,
        Finger::RM,
        Finger::RR,
        Finger::RP,
    ];
    pub fn as_usize(self) -> usize {
        match self {
            Finger::LP => 0,
            Finger::LR => 1,
            Finger::LM => 2,
            Finger::LI => 3,
            Finger::LT => 4,
            Finger::RT => 5,
            Finger::RI => 6,
            Finger::RM => 7,
            Finger::RR => 8,
            Finger::RP => 9,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum FingerKind {
    Pinky,
    Ring,
    Middle,
    Index,
    Thumb,
}

impl Finger {
    pub fn hand(self) -> Hand {
        match self {
            Finger::LP | Finger::LR | Finger::LM | Finger::LI | Finger::LT => Hand::Left,
            Finger::RP | Finger::RM | Finger::RR | Finger::RI | Finger::RT => Hand::Right,
        }
    }
    pub fn kind(self) -> FingerKind {
        match self {
            Finger::LT | Finger::RT => FingerKind::Thumb,
            Finger::LI | Finger::RI => FingerKind::Index,
            Finger::LM | Finger::RM => FingerKind::Middle,
            Finger::LR | Finger::RR => FingerKind::Ring,
            Finger::LP | Finger::RP => FingerKind::Pinky,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub col: u8,
    pub row: u8,
    pub layer: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyCoord {
    pub pos: Pos,
    pub x: f32,
    pub y: f32,
    pub finger: Finger,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Combo {
    pub coords: Vec<KeyCoord>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LayoutFormat {
    Fixed(Vec<Option<char>>),
    Flexible(Vec<KeyComponent>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyComponent {
    /// Vector describing the list of possible fingers the keys should
    /// be long to.
    pub finger: Vec<Finger>,
    /// Layer that the keys occupy.
    pub layer: u8,
    /// List of characters.
    pub keys: Vec<char>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LayoutData {
    pub name: String,
    pub authors: Vec<String>,
    pub note: Option<String>,
    #[serde(alias = "components")]
    pub format: LayoutFormat,
}

impl LayoutData {
    pub fn from_format(format: LayoutFormat) -> Self {
        LayoutData {
            name: "".to_string(),
            authors: vec![],
            note: None,
            format,
        }
    }
    pub fn flexible_from_keyboard_layout(kb: &Keyboard, layout: &Layout, corpus: &Corpus) -> Self {
        let mut components: Vec<KeyComponent> = vec![];
        let mut i = 0;
        for finger in &kb.keys.map {
            let mut chars: Vec<char> = vec![];
            for _ in finger {
                let key = corpus.uncorpus_unigram(layout.0[i]);
                if key != '\0' {
                    chars.push(key);
                }
                i += 1;
            }
            if !chars.is_empty() {
                components.push(KeyComponent {
                    finger: vec![finger[0].finger],
                    layer: finger[0].pos.layer,
                    keys: chars,
                });
            }
        }
        for combo in &kb.combos {
            let kc = &combo.coords[0];
            let key = corpus.uncorpus_unigram(layout.0[i]);
            i += 1;
            if key == '\0' {
                continue;
            }
            components.push(KeyComponent {
                finger: combo.coords.iter().map(|coord| coord.finger).collect(),
                layer: kc.pos.layer,
                keys: vec![key],
            });
        }
        Self::from_format(LayoutFormat::Flexible(components))
    }
    pub fn fixed_from_layout(layout: &Layout, corpus: &Corpus) -> Self {
        Self::from_format(LayoutFormat::Fixed(
            layout
                .0
                .iter()
                .map(|c| match c {
                    0 => None,
                    _ => Some(corpus.uncorpus_unigram(*c)),
                })
                .collect(),
        ))
    }
    pub fn name(self, name: String) -> Self {
        Self { name, ..self }
    }
    pub fn authors(self, authors: Vec<String>) -> Self {
        Self { authors, ..self }
    }
    pub fn note(self, note: String) -> Self {
        Self {
            note: Some(note),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    pub keys: FingerMap<Vec<KeyCoord>>,
    #[serde(default)]
    pub combos: Vec<Combo>,
    #[serde(skip)]
    pub combo_indexes: Vec<Vec<usize>>,
}

impl Keyboard {
    pub fn process_combo_indexes(&mut self) {
        self.combo_indexes = self
            .combos
            .iter()
            .map(|combo| {
                combo
                    .coords
                    .iter()
                    .map(|a| {
                        self.keys
                            .map
                            .iter()
                            .flatten()
                            .position(|b| a.pos.row == b.pos.row && a.pos.col == b.pos.col)
                            .expect("combo must use positions on layout")
                    })
                    .collect()
            })
            .collect();
    }
}

#[derive(Deserialize, Debug)]
pub struct Metric {
    pub name: String,
    pub short: String,
    pub ngram_type: NgramType,
}

#[derive(Deserialize)]
pub struct MetricData {
    pub metrics: Vec<Metric>,
    pub strokes: Vec<NstrokeData>,
    pub keyboard: Keyboard,
}

pub struct MetricContext {
    pub metrics: Vec<Metric>,
    pub keyboard: Keyboard,
    pub analyzer: Analyzer,
    pub layout: Layout,
}

impl MetricContext {
    pub fn layout_matrix(l: &LayoutData, kb: &Keyboard, corpus: &Corpus) -> Option<kc::Layout> {
        let kb_size = kb.keys.map.iter().flatten().count();
        match &l.format {
            LayoutFormat::Fixed(layout) => {
                if layout.len() <= kb_size {
                    Some(kc::Layout(
                        layout
                            .iter()
                            .take(kb_size)
                            .map(|c| match c {
                                Some(c) => corpus.corpus_char(*c),
                                None => 0,
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            LayoutFormat::Flexible(layout) => {
                let mut mapped_layout: FingerMap<Vec<char>> = FingerMap::default();
                let mut mapped_combos: Vec<char> = vec!['\0'; kb.combos.len()];
                for comp in layout {
                    if comp.finger.len() == 1 {
                        let finger = comp.finger[0];
                        for (i, c) in comp.keys.iter().enumerate() {
                            if i < kb.keys[finger].len() {
                                mapped_layout[finger].push(*c);
                                continue;
                            }
                            for (j, combo) in kb.combos.iter().enumerate() {
                                if mapped_combos[j] == '\0'
                                    && combo.coords.iter().any(|c| c.finger == finger)
                                {
                                    mapped_combos[j] = *c;
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                    'finger_searching: for finger in &comp.finger {
                        let l_len = mapped_layout[*finger].len();
                        let k_len = kb.keys[*finger].len();

                        if l_len < k_len {
                            mapped_layout[*finger].push(comp.keys[0]);
                            break;
                        }
                        for (i, combo) in kb.combos.iter().enumerate() {
                            if mapped_combos[i] == '\0'
                                && combo.coords.iter().any(|c| c.finger == *finger)
                            {
                                mapped_combos[i] = comp.keys[0];
                                break 'finger_searching;
                            }
                        }
                    }
                }

                for (mapped_finger, kb_finger) in
                    mapped_layout.map.iter_mut().zip(kb.keys.map.iter())
                {
                    mapped_finger.extend(vec!['\0'; kb_finger.len() - mapped_finger.len()]);
                }

                let matrix: Vec<CorpusChar> = mapped_layout
                    .map
                    .iter()
                    .flatten()
                    .chain(mapped_combos.iter())
                    .map(|c| corpus.corpus_char(*c))
                    .collect();

                if matrix.len() == kb_size + kb.combos.len() {
                    Some(kc::Layout(matrix))
                } else {
                    None
                }
            }
        }
    }

    pub fn set_layout(&mut self, l: &LayoutData) -> Option<()> {
        self.layout = MetricContext::layout_matrix(l, &self.keyboard, &self.analyzer.corpus)?;
        Some(())
    }

    pub fn fixed_layout_data(&self) -> LayoutData {
        LayoutData::fixed_from_layout(&self.layout, &self.analyzer.corpus)
    }

    pub fn flexible_layout_data(&self) -> LayoutData {
        LayoutData::flexible_from_keyboard_layout(
            &self.keyboard,
            &self.layout,
            &self.analyzer.corpus,
        )
    }

    pub fn new(l: &LayoutData, md: MetricData, corpus: Corpus) -> Option<Self> {
        let layout = MetricContext::layout_matrix(l, &md.keyboard, &corpus)?;
        let metric_data = KcMetricData::from(
            md.metrics.iter().map(|m| m.ngram_type).collect(),
            md.strokes,
            layout.0.len(),
        );
        let analyzer = Analyzer::from(metric_data, corpus);

        Some(Self {
            metrics: md.metrics,
            keyboard: md.keyboard,
            layout,
            analyzer,
        })
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FingerMap<T> {
    pub map: [T; 10],
}

impl<T> Index<Finger> for FingerMap<T> {
    type Output = T;

    fn index(&self, finger: Finger) -> &Self::Output {
        &self.map[finger.as_usize()]
    }
}

impl<T> IndexMut<Finger> for FingerMap<T> {
    fn index_mut(&mut self, finger: Finger) -> &mut Self::Output {
        &mut self.map[finger.as_usize()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn matrix() -> Keyboard {
        let mut map: [Vec<KeyCoord>; 10] = Default::default();
        for col in 0u8..10 {
            for row in 0u8..3 {
                let finger = match col {
                    4 => Finger::LI,
                    5 => Finger::RI,
                    _ => Finger::LIST[col as usize],
                };
                map[finger.as_usize()].push(KeyCoord {
                    pos: Pos { col, row, layer: 0 },
                    x: col.into(),
                    y: row.into(),
                    finger,
                });
            }
        }
        Keyboard {
            keys: FingerMap { map },
            combos: vec![],
            combo_indexes: Default::default(),
        }
    }

    fn corpus() -> Corpus {
        let mut char_list = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|c| vec![c, c.to_uppercase().next().unwrap()])
            .collect::<Vec<Vec<char>>>();
        char_list.extend(vec![
            vec![',', '<'],
            vec!['.', '>'],
            vec!['/', '?'],
            vec!['\'', '\"'],
            vec![';', ':'],
        ]);
        Corpus::with_char_list(char_list)
    }
    #[test]
    fn test_flexible() {
        let semimak: LayoutData =
            serde_json::from_str(&fs::read_to_string("test_data/flexible.json").unwrap()).unwrap();

        let corpus = corpus();

        let keyboard = matrix();
        let data = MetricData {
            strokes: vec![],
            metrics: vec![],
            keyboard,
        };

        let context = MetricContext::new(&semimak, data, corpus).unwrap();
        for (a, b) in context
            .layout
            .0
            .iter()
            .map(|c| context.analyzer.corpus.uncorpus_unigram(*c))
            .zip("fsxlrjhnbvtmzkq'cpwdgue,oa.yi/".chars())
        {
            assert_eq!(
                a,
                b,
                "{}",
                format!("layout matrix {:?} is wrong", context.layout.0)
            );
        }
        let new_data = context.flexible_layout_data();
        let LayoutFormat::Flexible(old_components) = semimak.format else {
            unreachable!()
        };
        let LayoutFormat::Flexible(new_components) = new_data.format else {
            unreachable!()
        };
        for (original, new) in old_components.iter().zip(new_components.iter()) {
            for (i, key) in new.keys.iter().enumerate() {
                if i >= original.keys.len() {
                    continue;
                }
                assert_eq!(*key, original.keys[i]);
            }
        }
    }
    #[test]
    fn test_fixed() {
        let semimak: LayoutData =
            serde_json::from_str(&fs::read_to_string("test_data/fixed.json").unwrap()).unwrap();

        let corpus = corpus();

        let keyboard = matrix();
        let data = MetricData {
            strokes: vec![],
            metrics: vec![],
            keyboard,
        };

        let context = MetricContext::new(&semimak, data, corpus).unwrap();
        for (a, b) in context
            .layout
            .0
            .iter()
            .map(|c| context.analyzer.corpus.uncorpus_unigram(*c))
            .zip("\0sxlrjhnbvtmzkq'cpwdgue,oa.yi/".chars())
        {
            assert_eq!(
                a,
                b,
                "{}",
                format!("layout matrix {:?} is wrong", context.layout.0)
            )
        }
    }
}
