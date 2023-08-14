use keymeow::{MetricContext, MetricData, LayoutData};
use kc::{self, Corpus};
use std::fs;
use serde_json;

pub fn main() {
    let semimak: LayoutData = serde_json::from_str(&fs::read_to_string("sample.json").unwrap()).unwrap();

    // println!("{:?}", semimak);

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
    // println!("{:?}", matrix);

    // for stroke in &mc.metric_data.strokes {
    //     let ngram: Vec<char> = layout.nstroke_chars(&stroke.nstroke).iter().map(|c| corpus.uncorpus_unigram(*c)).collect();
    //     let metrics: Vec<&String> = stroke.amounts.iter().map(|x| &mc.metrics[x.metric].short).collect();
    //     println!("{:?} {:?} {:?}",
    //     	 ngram,
    //     	 layout.frequency(&corpus, &stroke.nstroke, None),
    //     	 metrics);
    // }

    let analyzer = mc.analyzer(&corpus, &semimak);

    for (i, stat) in analyzer.stats.iter().enumerate() {
        let pw = 5.0 * stat / analyzer.layout.total_char_count(&corpus) as f32;
	println!("{}: {:.3}/w | every {:.2} words", mc.metrics[i].short, pw, 1.0 / pw);
    }
    
}
