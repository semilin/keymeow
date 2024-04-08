use kc::{self, Corpus, Swap};
use keymeow::{LayoutData, MetricContext, MetricData};
use rand::Rng;
use serde_json;
use std::fs;
use std::time::Instant;

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
            vec![';', ':'],
        ]);
        Corpus::with_char_list(char_list)
    };

    corpus.add_file("mr.txt").unwrap();

    let md: MetricData = serde_json::from_str(&fs::read_to_string("ansi.json").unwrap()).unwrap();
    let mut mc = MetricContext::new(&semimak, md, corpus).unwrap();
    mc.keyboard.process_combo_indexes();

    let mut rng = rand::thread_rng();

    let now = Instant::now();

    println!(
        "{:?}",
        mc.layout
            .matrix
            .iter()
            .map(|x| mc.analyzer.corpus.uncorpus_unigram(*x))
            .collect::<String>()
    );

    let oldstats: Vec<_> = mc.analyzer.calc_stats(&mc.layout);

    mc.layout.swap(&Swap::new(21, 23));

    let newstats: Vec<_> = mc.analyzer.calc_stats(&mc.layout);

    for (i, stat) in newstats.iter().enumerate() {
        let newp = stat / mc.layout.total_char_count(&mc.analyzer.corpus) as f32;
        let oldp = oldstats[i] / mc.layout.total_char_count(&mc.analyzer.corpus) as f32;
        println!(
            "{}: 1/{:.1} -> 1/{:.1}",
            mc.metrics[i].short,
            1.0 / oldp,
            1.0 / newp,
        );
    }

    println!("{:?}", mc.layout.matrix);

    // for stroke in mc.analyzer.data.strokes {
    // 	if stroke.amounts.iter().any(|m| m.metric == 1) {
    // 	    let v = match stroke.nstroke {
    // 		Nstroke::Monostroke(v) => vec![v],
    //             Nstroke::Bistroke(a) => a.to_vec(),
    //             Nstroke::Tristroke(a) => a.to_vec(),
    // 	    };
    // 	    let s: String = v.iter().map(|x| mc.analyzer.corpus.uncorpus_unigram(mc.analyzer.layouts[0].matrix[*x])).collect();
    // 	    println!("{}", s);
    // 	}
    // }
}
