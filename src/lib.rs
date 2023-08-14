use std::ops::{Index, IndexMut};
use serde::{Serialize, Deserialize};
use kc::{self, NgramType, Analyzer, Corpus, CorpusChar};

pub enum Hand {
    Left, Right
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Finger {
    LP, LR, LM, LI, LT,
    RT, RI, RM, RR, RP,
}

#[derive(Serialize, Deserialize)]
pub enum FingerKind {
    Pinky, Ring, Middle, Index, Thumb
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
    col: u8,
    row: u8,
    layer: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KeyCoord {
    pos: Pos,
    x: f32,
    y: f32,
    finger: Finger,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LayoutComponent {
    Key(KeyComponent),
    Chord(ChordComponent)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyComponent {
    /// Vector describing the list of possible fingers the keys should
    /// be long to.
    pub finger: Vec<Finger>,
    /// Layer that the keys occupy.
    pub layer: u8,
    /// List of characters.
    pub keys: Vec<char>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChordComponent {
    pub keys: Vec<char>,
    pub output: char,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutData {
    pub name: String,
    pub authors: Vec<String>,
    pub note: Option<String>,
    pub components: Vec<LayoutComponent>
}

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    pub keys: FingerMap<Vec<KeyCoord>>
}

#[derive(Deserialize, Debug)]
pub struct Metric {
    pub name: String,
    pub short: String,
    pub ngram_type: NgramType
}

#[derive(Deserialize)]
pub struct MetricData {
    pub metrics: Vec<Metric>,
    pub strokes: Vec<kc::NstrokeData>,
    pub keyboard: Keyboard
}

pub struct MetricContext {
    pub metric_data: kc::MetricData,
    pub metrics: Vec<Metric>,
    pub keyboard: Keyboard
}

impl MetricContext{
    pub fn from(md: MetricData) -> Self {
        Self {
            metric_data: kc::MetricData::from(md.metrics.iter()
					      .map(|x| x.ngram_type)
					      .collect(),
					      md.strokes,
					      md.keyboard.keys.map.iter().flatten().count()),
            metrics: md.metrics,
            keyboard: md.keyboard
        }
    }
    pub fn analyzer<'a>(&'a self, corpus: &'a Corpus, l: &'a LayoutData) -> Analyzer<'a> {
        let mut mapped_layout: FingerMap<Vec<char>> = FingerMap::default();
        for component in &l.components {
            match component {
                LayoutComponent::Key(comp) => {
                    if comp.finger.len() == 1 {
                        let finger = comp.finger[0];
                        for (i, c) in comp.keys.iter().enumerate() {
                            if i < self.keyboard.keys[finger].len() {
                                mapped_layout[finger].push(*c)
                            }
                        }
                        continue;
                    }
                    for finger in &comp.finger {
                        let l_len = mapped_layout[*finger].len();
                        let k_len = self.keyboard.keys[*finger].len();
                        // println!("{} {}", l_len, k_len);
                        if l_len < k_len {
                            mapped_layout[*finger].push(comp.keys[0]);
                            break;
                        }
                    }
                },
	        // chord support will be added later
                _ => todo!()
            }
        }

        let matrix: Vec<CorpusChar> = mapped_layout.map
	    .iter()
            .flatten()
	    .map(|c| *corpus.corpus_char(*c).unwrap())
	    .collect();

        let layout = kc::Layout { matrix };
        Analyzer::from(&self.metric_data, &corpus, layout)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FingerMap<T> {
    pub map: [T; 10]
}

impl<T> Index<Finger> for FingerMap<T> {
    type Output = T;
    
    fn index(&self, finger: Finger) -> &Self::Output {
        match finger {
            Finger::LP => &self.map[0],
            Finger::LR => &self.map[1],
            Finger::LM => &self.map[2],
            Finger::LI => &self.map[3],
            Finger::LT => &self.map[4],
            Finger::RT => &self.map[5],
            Finger::RI => &self.map[6],
            Finger::RM => &self.map[7],
            Finger::RR => &self.map[8],
            Finger::RP => &self.map[9],
        }
    }
}

// TODO: dry
impl<T> IndexMut<Finger> for FingerMap<T> {
    fn index_mut(&mut self, finger: Finger) -> &mut Self::Output {
        match finger {
            Finger::LP => &mut self.map[0],
            Finger::LR => &mut self.map[1],
            Finger::LM => &mut self.map[2],
            Finger::LI => &mut self.map[3],
            Finger::LT => &mut self.map[4],
            Finger::RT => &mut self.map[5],
            Finger::RI => &mut self.map[6],
            Finger::RM => &mut self.map[7],
            Finger::RR => &mut self.map[8],
            Finger::RP => &mut self.map[9],
        }
    }
}
