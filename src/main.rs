extern crate graph_lib;
use graph_lib::graph::Graph;
use std::time::{Duration, Instant};

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, Read};
use std::path::Path;
use graph_lib::structures::*;
use std::fmt::Debug;
use std::hash::Hash;


fn dump_graph_list<T: Eq + Hash + Debug + Send + Clone + Ord + Sync>(list: &Tier<T>, n: usize) {
    if list.count_chunks() == 0 {
        return
    }
    println!("{:05?}", list.map.values().next().unwrap().labeling_sorted);
    let path_str = format!("out/{}.txt", n);
    let path = Path::new(&path_str);
    let path_pretty = path.display();
    fs::create_dir("out");
    let file = match File::create(&path) {
        Err(e) => panic!("couldn't create {} : {}", path_pretty, e.description()),
        Ok(file) => file,
    };
    
    let mut writer = BufWriter::new(&file);
    for chunk in list.map.values() {
        let mut buf = String::new();
        let (last, graphs) = chunk.checked.split_last().unwrap();
        for graph in graphs.iter() {
            graph.inner.to_string_append(&mut buf);
            buf.push_str(";");
        }
        last.inner.to_string_append(&mut buf);
        buf.push_str("\n");
        writer.write(&buf.into_bytes());
        
    }
    writer.flush();
}
fn fmt_dur(d: &Duration) -> String {
    let hours = d.as_secs() / 3600;
    let mins = d.as_secs() / 60 % 60;
    format!(
        "{:04}:{:02}:{:02}.{:03}",
        hours,
        mins,
        d.as_secs() % 60,
        d.subsec_nanos() / 1_000_000
    )
}


use graph_lib::graph_like::GraphLike;
fn main() {
    let root = graph_lib::graph::Graph::new(1);
    let root_tier = graph_lib::structures::Tier::from_graph(root, |_| vec!(0));
    let mut tiers = vec![root_tier];
    for i in 0..20 {
        let start = Instant::now();
        tiers.push(tiers[i].generate_next_size(&|g: &Graph, b: &mut Vec<Vec<(u32, u32)>>| g.label(b)));
        println!(
            "There are {:?} graphs on {} vertices, distinguished into {} classes, generated in {}s",
            tiers[i + 1].count_graphs(),
            i + 2,
            tiers[i + 1].count_chunks(),
            start.elapsed().as_secs(),
        );
        if tiers[i + 1].count_graphs().0 == 0 {
            println!("stopping");
            return
        }
        println!("Writing to disk...");
        dump_graph_list(&tiers[i + 1], i + 2);
        if i > 1 {
            tiers[i].map.clear();
            tiers[i].map.shrink_to_fit();
        }
        println!("Done. Starting next tier");
    }
    /*for i in 2..11 {
        read_graph_tier(i);
    }*/
}

use std::collections::HashMap;
use std::io::BufRead;

fn read_graph_tier(n: u32) -> HashMap<Vec<u32>, u32> {
    let mut out = HashMap::new();
    let path_str = format!("out/{}.txt", n);
    let path = Path::new(&path_str);
    let path_pretty = path.display();

    let file = match File::open(&path) {
        Err(e) => panic!("couldn't open {} : {}", path_pretty, e.description()),
        Ok(file) => file,
    };
    
    let mut reader = BufReader::new(&file);
    let mut buf = vec!(Vec::with_capacity(n as usize); n as usize);
    for line in reader.lines() {
        for graph_b64 in line.unwrap().split(|c| c == ';' || c == '{' || c == '}') {
            if graph_b64.len() < 2 {
                continue
            }
            let g = Graph::from_str(&graph_b64, n as usize).unwrap();
            let mut l = g.label(&mut buf);
            l.sort();
            *out.entry(l).or_insert(0) += 1;
            //println!("{:?}", g.label(&mut buf));
        }
    }
    let mut histo = HashMap::new();
    let mut chunks = 0u32;
    for chunk in out.iter() {
        chunks += 1;
        *histo.entry(chunk.1).or_insert(0) += 1;
    }
    let mut results = vec!();

    for v in histo.iter() {
        results.push((v.0, v.1));
    }
    results.sort();

    for v in results.iter() {
        println!("There are {} chunks with {} graphs", v.1, v.0);
    }
    println!("{}", chunks);
    out
}
