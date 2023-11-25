use kc::{self, Corpus, Swap};
use keymeow::{LayoutData, MetricContext, MetricData};
use serde_json;
use std::{fs};

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
        Corpus::with_char_list(&mut char_list)
    };

    corpus.add_file("tr.txt").unwrap();

    let md: MetricData =
        serde_json::from_str(&fs::read_to_string("combo_test.json").unwrap()).unwrap();
    let mut mc = MetricContext::new(&semimak, md, corpus).unwrap();
    mc.keyboard.process_combo_indexes();

    // let mut rng = rand::thread_rng();

    // let now = Instant::now();
    // for _ in 1..20000 {
    // 	mc.analyzer.swap(0, Swap::new(rng.gen_range(0..30), rng.gen_range(0..30)));
    // }
    // println!("{:?}", now.elapsed().as_millis());

    // println!("{:?}", mc.analyzer.layouts[0].matrix.iter().map(|x| mc.analyzer.corpus.uncorpus_unigram(*x)).collect::<String>());

    let old: Vec<_> = mc.analyzer.stats.iter().cloned().enumerate().collect();

    mc.analyzer.swap(0, &Swap::new(21, 23), false);

    for (i, stat) in mc.analyzer.stats.iter().enumerate() {
        let newp = stat / mc.analyzer.layouts[0].total_char_count(&mc.analyzer.corpus) as f32;
	let oldp = old[i].1 / mc.analyzer.layouts[0].total_char_count(&mc.analyzer.corpus) as f32;
        println!(
            "{}: 1/{:.1} -> 1/{:.1}",
            mc.metrics[i].short,
            1.0/oldp,
	    1.0/newp,
	);
    }

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
