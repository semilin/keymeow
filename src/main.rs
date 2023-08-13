use keymeow::{MetricContext, MetricData, Metric, LayoutData, LayoutComponent, KeyComponent, Finger, Keyboard, FingerMap};
use kc::{self, Analyzer, Corpus, CorpusChar};
use std::fs;
use serde_json;

pub fn main() {
    let semimak: LayoutData = serde_json::from_str(&fs::read_to_string("sample.json").unwrap()).unwrap();

    println!("{:?}", semimak);

    let md: MetricData = serde_json::from_str(&fs::read_to_string("matrix.json").unwrap()).unwrap();
    let mc = MetricContext::from(md);

    let mut corpus = {
        let mut char_list = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|c| vec![c, c.to_uppercase().next().unwrap()])
            .collect::<Vec<Vec<char>>>();
        char_list.extend(vec![vec![',', '<'],
			      vec!['.', '>'],
			      vec!['/', '?'],
			      vec!['\'', '\"']]);
        Corpus::with_char_list(char_list)
    };

    corpus.add_file("tr.txt").unwrap();

    let mut mapped_layout: FingerMap<Vec<char>> = FingerMap::default();
    for component in semimak.components {
        match component {
            LayoutComponent::Key(comp) => {
                if comp.finger.len() == 1 {
                    let finger = comp.finger[0];
                    for (i, c) in comp.keys.iter().enumerate() {
                        if i < mc.keyboard.keys[finger].len() {
                            mapped_layout[finger].push(*c)
                        }
                    }
                    continue;
                }
                for finger in comp.finger {
                    let l_len = mapped_layout[finger].len();
                    let k_len = mc.keyboard.keys[finger].len();
                    println!("{} {}", l_len, k_len);
                    if l_len < k_len {
                        mapped_layout[finger].push(comp.keys[0]);
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

    let layout = kc::Layout { matrix: matrix.clone() };

    println!("{:?}", matrix);

    for stroke in &mc.metric_data.strokes {
	let ngram: Vec<char> = layout.nstroke_chars(&stroke.nstroke).iter().map(|c| corpus.uncorpus_unigram(*c)).collect();
	let metrics: Vec<&String> = stroke.amounts.iter().map(|x| &mc.metrics[x.metric].short).collect();
	println!("{:?} {:?} {:?}",
		 ngram,
		 layout.frequency(&corpus, &stroke.nstroke, None),
		 metrics);
    }

    let analyzer = Analyzer::from(&mc.metric_data, &corpus, kc::Layout { matrix });
    for (i, stat) in analyzer.stats.iter().enumerate() {
	println!("{}: {:.2}%", mc.metrics[i].short, 100.0 * stat / layout.total_char_count(&corpus) as f32);
    }
    
}
