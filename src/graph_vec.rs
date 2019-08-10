// The bare bones of a graph structure and functionality

use base64;
use crate::bitvec::*;

use crate::graph_like::GraphLike;

#[derive(Clone, Debug)]
pub struct Graph {
    pub num_verts: usize,
    pub edges: Vec<bool>,
}

impl GraphLike for Graph {

    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn get_edge(&self, n: usize, m: usize) -> bool {
        let n1 = m.min(n);
        let m1 = m.max(n);
        self.edges[(m1 * m1 - m1) / 2 + n1]
    }
}


impl Graph {
    pub fn new(n: usize) -> Self {
        let num_edges = n * (n - 1) / 2;
        let mut e = Vec::with_capacity(num_edges);
        for _ in 0..num_edges {
            e.push(false);
        }

        Graph {
            num_verts: n,
            edges: e,
        }
    }

    pub fn get_next_size(&self) -> Vec<Graph> {
        let mut next_size = Vec::new();
        for mut i in 0..(1 << self.num_verts) {
            let mut g = self.clone();
            g.num_verts += 1;
            for _ in 0..self.num_verts {
                g.edges.push((i & 1) != 0);
                i >>= 1;
            }

            next_size.push(g);
        }
        next_size
    }

    // base 64 encoding of bits of edges
    pub fn to_string(&self) -> String {
        let mut bitvec = BitVec::new();
        for edge in self.edges.iter() {
            bitvec.push(*edge);
        }
        base64::encode_config(&bitvec.vec, base64::STANDARD_NO_PAD)
    }

    pub fn to_string_append(&self, buf: &mut String) {
        let mut bitvec = BitVec::new();
        for edge in self.edges.iter() {
            bitvec.push(*edge);
        }
        base64::encode_config_buf(&bitvec.vec, base64::STANDARD_NO_PAD, buf);
    }

    // decode base 64, size hint needed for space efficiency
    pub fn from_str(s: &str, num_verts: usize) -> Result<Graph, Box<dyn std::error::Error>> {
        let bits = base64::decode(s)?;

        let mut edges = Vec::new();
        let num_edges = num_verts * (num_verts - 1) / 2;

        for bit_idx in 0..num_edges {
            let bit = bit_idx % 8;
            let byte = bit_idx / 8;
            edges.push(bits[byte] & (1 << bit) > 0);
        }
        Ok(Graph { num_verts, edges })
    }

    pub fn label_neighbors(&self) -> Vec<u32> {
        let mut l = Vec::new();
        for i in 0..self.num_verts {
            let mut k = 0;
            for j in 0..self.num_verts {
                if self.get_edge(i as usize, j as usize) {
                    k += 1;
                }
            }
            l.push(k);
        }
        l
    }

    //counts red k3's that each vertex is a part of
    pub fn label_k3s(&self) -> Vec<u32> {
        let mut ans = Vec::with_capacity(self.num_verts);

        for v in 0..self.num_verts {
            let mut count = 0u32;
            for a in 0..self.num_verts {
                if a != v && self.get_edge(v, a) {
                    for b in a + 1..self.num_verts {
                        if b != v && b != a && self.get_edge(v, b) && self.get_edge(a, b) {
                            count += 1;
                        }
                    }
                }
            }
            ans.push(count);
        }
        ans
    }
    //TODO LazyStatic this guy
    //    const FULLY_CONNECTED_VERTEX_LABEL: u32 =
    pub fn label(&self) -> Vec<u32> {
        if self.num_verts < 2 {
            return vec![0];
        }
        let mut l: Vec<u32> = Vec::new();
        for i in 0..self.num_verts {
            let mut k = 0;
            for j in 0..self.num_verts {
                if i != j && self.get_edge(i as usize, j as usize) {
                    k += 1;
                }
            }
            l.push(k);
        }
        let mut m: Vec<u32> = Vec::new();
        for i in 0..self.num_verts {
            let mut k = 0;
            for j in 0..self.num_verts {
                if i != j && self.get_edge(i as usize, j as usize) {
                    k += l[j as usize];
                }
            }
            m.push(k);
        }
        let t = self.label_k3s();
        let mut n: Vec<u32> = Vec::new();
        for ((x, y), z) in l.into_iter().zip(m).zip(t) {
            n.push((1 << 31) as u32 + (x << 20) as u32 + (y << 8) as u32 + z);
        }
        /*
        let mut copy = n.clone();
        copy.sort();
        let comp = compute_complexity(&copy);
        if comp > 1000 {
            let t = self.label_k3s();
            for (x, y) in n.iter_mut().zip(t) {
                *x += y;
            }
        }*/
        n
    }

}

use crate::util;
fn compute_complexity(labeling_sorted: &[u32]) -> u64 {
    let mut comp = 1;
    let mut current = labeling_sorted[0];
    let mut cur_streak = 1;
    for i in 1..labeling_sorted.len() {
        if labeling_sorted[i] == current {
            cur_streak += 1;
        } else {
            current = labeling_sorted[i];
            comp *= util::factorial(&cur_streak);
            cur_streak = 1;
        }
    }
    comp *= util::factorial(&cur_streak);
    //println!("{} {:?}", comp, labeling_sorted);
    comp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_size() {
        let root = Graph::new(2);
        let next_size = root.get_next_size();
        assert_eq!(next_size.len(), 4);
    }

    #[test]
    fn serialize() {
        let edges = vec![0b1101_0011; 6];
        let edges = BitVec {
            len: 45,
            vec: edges,
        };
        let graph = Graph {
            num_verts: 10,
            edges,
        };
        assert_eq!(graph.to_string(), "09PT09PT");
    }

    #[test]
    fn round_trip() {
        let edges = vec![0b1101_0011; 6];
        let edges = BitVec {
            len: 45,
            vec: edges,
        };
        let graph = Graph {
            num_verts: 10,
            edges,
        };
        let ser = graph.to_string();
        let des = Graph::from_str(&ser, 10).unwrap();
        assert_eq!(graph.edges.vec, des.edges.vec);
    }
}
