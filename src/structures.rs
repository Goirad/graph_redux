use crate::graph::Graph;
use crate::util;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use crate::graph_like::GraphLike;

#[derive(Clone, Debug)]
pub struct Chunk<T: Hash> {
    pub comp: u64,
    pub labeling_sorted: ChunkLabeling<T>,
    pub checked: Vec<LabeledGraph>,
    pub unchecked: Vec<Option<LabeledGraph>>,
}

impl<T: Hash> Default for Chunk<T> {
    fn default() -> Self {
        Chunk {
            comp: 0,
            labeling_sorted: ChunkLabeling(vec![]),
            checked: vec![],
            unchecked: vec![],
        }
    }
}

impl<T: Eq + Hash> Chunk<T> {
    pub fn clean_isos(&mut self) {
        //two stages, first every checked with every unchecked,
        //then remaining unchecked amongst themselves
        //Stage 1
        for checked_idx in (0..self.checked.len()).rev() {
            for unchecked in self.unchecked.iter_mut() {
                if let Some(g) = unchecked {
                    if is_color_iso(&self.checked[checked_idx], &g) {
                        *unchecked = None;
                    }
                }
            }
        }

        //Stage 2
        //unchecked now contains a mix of Somes and Nones
        for i in 0..self.unchecked.len() {
            if let Some(g) = self.unchecked[i].take() {
                for unchecked in self.unchecked.iter_mut().skip(i) {
                    if let Some(h) = unchecked {
                        if is_color_iso(&g, h) {
                            *unchecked = None;
                        }
                    }
                }
                self.checked.push(g);
            }
        }
        self.unchecked.clear();
    }
    
    // returns child graphs that satisfy F
    pub fn get_next_size<F: Fn(&Graph) -> bool>(self, filter: F) -> Vec<Graph> {
        let mut out = vec![];
        for graph in self.checked.iter() {
            out.append(
                &mut graph
                    .inner
                    .get_next_size()
                    .into_iter()
                    .filter(|g| filter(g))
                    .collect(),
            );
        }
        out
    }

    pub fn from_sorted_label(l: ChunkLabeling<T>) -> Self {
        let mut out = Chunk::default();
        out.comp = compute_complexity(&l);
        out.labeling_sorted = l;
        out
    }

    pub fn trim(&mut self) {
        self.checked.shrink_to_fit();
        self.unchecked.shrink_to_fit();
    }
}

#[derive(Clone, Debug)]
pub struct LabeledGraph {
    pub inner: Graph,
    pub labels: GraphLabeling,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ChunkLabeling<T: Hash>(Vec<T>);

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
// label of vertex i = chunklabel[graphlabel[i]]
pub struct GraphLabeling(Vec<u8>);

#[derive(Debug)]
pub struct Tier<T: Hash + Debug + Eq + Send> {
    pub map: HashMap<ChunkLabeling<T>, Chunk<T>>,
}

impl<T: Hash + Debug + Eq + Send> Default for Tier<T> {
    fn default() -> Self {
        Tier {
            map: HashMap::new(),
        }
    }
}

impl<T: Hash + Debug + Eq + Clone + Ord + Send + Sync> Tier<T> {
    pub fn count_chunks(&self) -> usize {
        self.map.len()
    }
    pub fn count_graphs(&self) -> (usize, usize) {
        self.map
            .values()
            .fold((0, 0), |(checked, unchecked), chunk| {
                (
                    checked + chunk.checked.len(),
                    unchecked + chunk.unchecked.len(),
                )
            })
    }
    pub fn count_unchecked(&self) -> (usize, usize) {
        self.map.values().fold((0, 0), |(len, cap), chunk| {
            (
                len + chunk.unchecked.len(),
                cap + chunk.unchecked.capacity(),
            )
        })
    }
    pub fn from_graph<F: Fn(&Graph) -> Vec<T>>(g: Graph, f: F) -> Self {
        let mut out = Tier::default();
        let mut label = f(&g);
        let mut label_sorted = label.clone();
        label_sorted.sort();
        let label = normalize(&mut label, &label_sorted);
        let label_sorted1 = ChunkLabeling(label_sorted);
        let label_sorted2 = label_sorted1.clone();

        let out_chunk = out
            .map
            .entry(label_sorted1)
            .or_insert_with(|| Chunk::from_sorted_label(label_sorted2));

        out_chunk.checked.push(LabeledGraph {
            inner: g,
            labels: GraphLabeling(label),
        });
        out
    }
    pub fn insert_checked<F: Fn(&Graph) -> Vec<T>>(&mut self, g: Graph, f: F) {
        let label = f(&g);
        let mut label_sorted = label.clone();
        let mut label = label;
        label_sorted.sort();
        let label = normalize(&mut label, &label_sorted);
        let label_sorted1 = ChunkLabeling(label_sorted);
        let label_sorted2 = label_sorted1.clone();

        let out_chunk = self
            .map
            .entry(label_sorted1)
            .or_insert_with(|| Chunk::from_sorted_label(label_sorted2));

        out_chunk.checked.push(LabeledGraph {
            inner: g,
            labels: GraphLabeling(label),
        });
    }

    //FIXME: This needs two functions, a filter for generating next size and a labeling function
    pub fn generate_next_size<L: Sync + Fn(&Graph, &mut Vec<Vec<(u32, u32)>>) -> Vec<T>>(&self, labeler: &L) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::{Arc, Mutex};
        let mut out: Arc<Mutex<Tier<T>>> = Arc::new(Mutex::new(Tier::default()));
        let mut count = &AtomicU64::new(0);
        let bar = &ProgressBar::new(self.count_chunks() as u64);
        let mut clean_count = AtomicU64::new(0);
        let mut prev_g_count = AtomicU64::new(0);
        let cycles = &AtomicU64::new(0);
        const NUM_THREADS: usize = 4;
        let working = &AtomicU64::new(NUM_THREADS as u64);
        let new_num_verts = self.map.iter().next().unwrap().1.checked[0].inner.num_verts() + 1;
        rayon::scope(|s| {
            for i in 0..NUM_THREADS {
                let out = out.clone();
                s.spawn(move |_| {
                    let mut temp = Tier::default();
                    let mut label_buffer = vec![Vec::with_capacity(new_num_verts); new_num_verts];
                    for chunk in self.map.values().skip(i).step_by(NUM_THREADS) {
                        for graph in chunk.checked.iter() {
                            //This is producing a huge vec that we then iterate over
                            // (2^num_verts) graphs to be precise
                            let descendants = graph.inner.get_next_size();
                            for descendant in descendants.into_iter() {
                                if !descendant.has_kns(4, 4) {
                                    let mut label = labeler(&descendant, &mut label_buffer);
                                    let mut label_sorted = label.clone();
                                    label_sorted.sort();
                                    let label = normalize(&mut label, &label_sorted);
                                    let label_sorted1 = ChunkLabeling(label_sorted);
                                    let label_sorted2 = label_sorted1.clone();
                                    count.fetch_add(1, Ordering::Relaxed);

                                    let out_chunk = temp
                                        .map
                                        .entry(label_sorted1)
                                        .or_insert_with(|| Chunk::from_sorted_label(label_sorted2));

                                    out_chunk.unchecked.push(Some(LabeledGraph {
                                        inner: descendant,
                                        labels: GraphLabeling(label),
                                    }));
                                }
                            }
                        }
                        bar.inc(1);
                        //after some threshold, dump the temp map into the out tier
                        if temp.count_unchecked().0 > 500_000 {
                            let mut out = out.lock().unwrap();
                            for mut c in temp.map.into_iter() {
                                let out_chunk = out
                                    .map
                                    .entry(c.0.clone())
                                    .or_insert_with(|| Chunk::from_sorted_label(c.0.clone()));
                                out_chunk.unchecked.append(&mut c.1.unchecked);
                            }
                            cycles.fetch_add(1, Ordering::Relaxed);
                            temp = Tier::default();
                        }
                    }

                    //FIXME: deduplicate from above code
                    //This happens at the end to make sure all graphs get moved over
                    let mut out = out.lock().unwrap();
                    for mut c in temp.map.into_iter() {
                        let out_chunk = out
                            .map
                            .entry(c.0.clone())
                            .or_insert_with(|| Chunk::from_sorted_label(c.0.clone()));
                        out_chunk.unchecked.append(&mut c.1.unchecked);
                    }
                    working.fetch_sub(1, Ordering::Relaxed);
                });
            }
            let out = out.clone();
            s.spawn(move |_| {
                let mut last = cycles.load(Ordering::Relaxed);
                while working.load(Ordering::Relaxed) > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    let new = cycles.load(Ordering::Relaxed);
                    if new > last + 10 {
                        last = new;
                        let tp = rayon::ThreadPoolBuilder::default().build().unwrap();
                        tp.install(|| {
                            let mut out = out.lock().unwrap();
                            out.map.values_mut().par_bridge().for_each(|chunk| {
                                chunk.clean_isos();
                            });

                            let cs = out.count_chunks();
                            let gs = out.count_graphs();
                            clean_count.fetch_add(1, Ordering::Relaxed);
                            println!(
                                "{} graphs and {} chunks\n+{} graphs, cleaning {}\n",
                                gs.0,
                                cs,
                                gs.0 as u64 - prev_g_count.load(Ordering::Relaxed),
                                clean_count.load(Ordering::Relaxed),
                            );
                            prev_g_count.store(gs.0 as u64, Ordering::Relaxed);
                        });
                    }
                }
            });
        });
        bar.finish();
        let mut out = Arc::try_unwrap(out).unwrap().into_inner().unwrap();
        let before = out.count_chunks();
        println!("final cleanup");
        out.map.values_mut().par_bridge().for_each(|chunk| {
            if chunk.comp > 10_000_000 && chunk.unchecked.len() > 0 {
                println!(
                    "cleaning {}, {}, comp {}",
                    chunk.checked.len(),
                    chunk.unchecked.len(),
                    chunk.comp
                );
            }

            chunk.clean_isos();
            chunk.trim();
        });
        assert!(out.count_chunks() == before);
        out
    }
}

fn compute_complexity<T: Eq + Hash>(labeling_sorted: &ChunkLabeling<T>) -> u64 {
    let labeling_sorted = &labeling_sorted.0;
    let mut comp = 1u64;
    let mut current = &labeling_sorted[0];
    let mut cur_streak = 1;
    for i in 1..labeling_sorted.len() {
        if labeling_sorted[i] == *current {
            cur_streak += 1;
        } else {
            current = &labeling_sorted[i];
            comp *= util::factorial(&cur_streak);
            cur_streak = 1;
        }
    }
    comp *= util::factorial(&cur_streak);
    //println!("{} {:?}", comp, labeling_sorted);
    comp
}

//no element in either should be <= length of the lists
fn normalize<T: Eq>(unsorted: &[T], sorted: &[T]) -> Vec<u8> {
    let mut out = vec!(0; unsorted.len());
    for (i, label) in unsorted.iter().enumerate() {
        for (j, s) in sorted.iter().enumerate() {
            if label == s {
                out[i] = j as u8;
            }
        }
    }
    out
}

fn collapse_verts(v: &Vec<Vec<usize>>, out: &mut Vec<usize>) {
    out.clear();
    for i in v.iter() {
        for j in i.iter() {
            out.push(*j);
        }
    }
}

fn compare(
    g: &LabeledGraph,
    h: &LabeledGraph,
    n: usize, //how many verts to compare
    verts_g: &mut Vec<Vec<usize>>,
    collapsed_verts_g: &mut Vec<usize>,
    collapsed_verts_h: &Vec<usize>,
) -> bool {
    collapse_verts(&verts_g, collapsed_verts_g);
    for i in 0..(n - 1) {
        for j in i + 1..n {
            if g.inner
                .get_edge(collapsed_verts_g[i as usize], collapsed_verts_g[j as usize])
                != h.inner
                    .get_edge(collapsed_verts_h[i as usize], collapsed_verts_h[j as usize])
            {
                return false;
            }
        }
    }
    return true;
}

fn permute(
    depth_to_now: usize,
    sub_depth: usize,
    depth: usize,
    orig_verts_g: &Vec<Vec<usize>>,
    verts_g: &mut Vec<Vec<usize>>,
    collapsed_verts_g: &mut Vec<usize>,
    collapsed_verts_h: &Vec<usize>,
    g: &LabeledGraph,
    h: &LabeledGraph,
) -> bool {
    if sub_depth == verts_g[depth].len() {
        return rec_iso_check(
            depth + 1,
            orig_verts_g,
            verts_g,
            collapsed_verts_h,
            collapsed_verts_g,
            g,
            h,
        );
    }
    if depth_to_now + sub_depth > 1
        && !compare(
            g,
            h,
            depth_to_now + sub_depth,
            verts_g,
            collapsed_verts_g,
            collapsed_verts_h,
        )
    {
        /*println!(
            "broke out at sub depth {} and depth {}, checked {}",
            sub_depth,
            depth,
            depth_to_now + sub_depth
        );*/
        return false;
    }
    //remember what the partition looked like before we and everyone else messed with it
    let c = verts_g[depth].clone();
    //println!("{:?}", &c);
    for i in sub_depth..verts_g[depth].len() {
        verts_g[depth] = c.clone();
        verts_g[depth].swap(sub_depth, i);
        //println!("{:?} sd: {} i: {}", &verts_g[depth], sub_depth, i);
        if permute(
            depth_to_now,
            sub_depth + 1,
            depth,
            orig_verts_g,
            verts_g,
            collapsed_verts_g,
            collapsed_verts_h,
            g,
            h,
        ) {
            return true;
        }
    }
    return false;
}

pub fn rec_iso_check(
    depth: usize,
    orig_verts_g: &Vec<Vec<usize>>,
    verts_g: &mut Vec<Vec<usize>>,
    collapsed_verts_h: &Vec<usize>,
    collapsed_verts_g: &mut Vec<usize>,
    g: &LabeledGraph,
    h: &LabeledGraph,
) -> bool {
    if depth >= g.inner.num_verts() {
        let res = compare(
            g,
            h,
            g.inner.num_verts(),
            verts_g,
            collapsed_verts_g,
            collapsed_verts_h,
        );
        //println!("{:?}", &verts_g);
        //println!("bottomed out on {:?}, res = {}", collapsed_verts_g, res);
        res
    } else {
        //sum of the lengths of partitions preceding the one we are currently working on
        let depth_to_now = orig_verts_g
            .iter()
            .take(depth)
            .fold(0, |acc, partition| acc + partition.len());

        return permute(
            depth_to_now,
            0,
            depth,
            orig_verts_g,
            verts_g,
            collapsed_verts_g,
            collapsed_verts_h,
            g,
            h,
        );
    }
}

// probably have a different function for when all vertices are indistinguishable
pub fn is_color_iso(g: &LabeledGraph, h: &LabeledGraph) -> bool {
    //println!("starting comparison ==============================================");

    //TODO normalize vertex labelings
    let n = g.inner.num_verts();
    let mut orig_verts_g = vec![Vec::with_capacity(n); n];
    let mut verts_g = vec![Vec::with_capacity(n); n];
    let mut verts_h = vec![Vec::with_capacity(n); n];
    //TODO EXTREMELY IMPORTANT
    //verify that partitions are sorted so that larger partitions appear
    //later in the array, this is to minimimize entering and exiting the
    //recursive function

    for i in 0..g.inner.num_verts() {
        //println!("{}", i);
        orig_verts_g[g.labels.0[i as usize] as usize].push(i as usize);
        verts_g[g.labels.0[i as usize] as usize].push(i as usize);
        verts_h[h.labels.0[i as usize] as usize].push(i as usize);
    }
    orig_verts_g.sort_by(|a, b| a.len().cmp(&b.len()));
    verts_g.sort_by(|a, b| a.len().cmp(&b.len()));
    verts_h.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut collapsed_verts_h = Vec::new();
    collapse_verts(&verts_h, &mut collapsed_verts_h);
    let mut collapsed_verts_g = Vec::new();

    //start is the index of the first partition with more than one vertex
    let mut start = verts_g.len();
    while start > 0 && verts_g[start - 1].len() > 1 {
        start -= 1;
    }

    //Once the arrays are sorted, we can skip to the first depth that has len > 1
    //println!("{:?}", vertsO_g);
    rec_iso_check(
        start,
        &orig_verts_g,
        &mut verts_g,
        &collapsed_verts_h,
        &mut collapsed_verts_g,
        g,
        h,
    )
}
