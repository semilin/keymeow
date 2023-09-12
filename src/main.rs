use kc::{self, Corpus};
use keymeow::{LayoutData, MetricContext, MetricData};
use serde_json;
use std::fs;

pub fn main() {
    let semimak: LayoutData =
        serde_json::from_str(&fs::read_to_string("sample.json").unwrap()).unwrap();

    // println!("{:?}", semimak);

    let mut corpus = {
        let mut char_list = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|c| vec![c, c.to_uppercase().next().unwrap()])
            .collect::<Vec<Vec<char>>>();
        char_list.extend(vec![
            vec![',', '<'],
            vec!['.', '>'],
            vec!['/', '?'],
            vec!['\'', '\"'],
        ]);
        Corpus::with_char_list(char_list)
    };

    corpus.add_file("tr.txt").unwrap();

    let md: MetricData = serde_json::from_str(&fs::read_to_string("matrix.json").unwrap()).unwrap();
    let mc = MetricContext::new(&semimak, md, corpus);

    for (i, stat) in mc.analyzer.stats.iter().enumerate() {
        let pw = 5.0 * stat / mc.analyzer.layout.total_char_count(&mc.analyzer.corpus) as f32;
        println!(
            "{}: {:.3}/w | every {:.2} words",
            mc.metrics[i].short,
            pw,
            1.0 / pw
        );
    }
}
