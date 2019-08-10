// The bare bones of a graph structure and functionality

use crate::bitvec::BitVec;
use base64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::graph_like::GraphLike;

#[derive(Clone, Debug)]
pub struct Graph {
    num_verts: usize,
    edges: BitVec,
}

impl GraphLike for Graph {
    fn num_verts(&self) -> usize {
        self.num_verts
    }

    fn get_edge(&self, n: usize, m: usize) -> bool {
        let n1 = m.min(n);
        let m1 = m.max(n);
        self.edges.get((m1 * m1 - m1) / 2 + n1)
    }
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let num_edges = n * (n - 1) / 2;
        let mut e = BitVec::new();
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

    //counts red k3's that each vertex is a part of
    pub fn label_k3s(&self, col: bool) -> Vec<u32> {
        let mut ans = Vec::with_capacity(self.num_verts);

        for v in 0..self.num_verts {
            let mut count = 0u32;
            for a in 0..self.num_verts {
                if a != v && (self.get_edge(v, a) == col) {
                    for b in a + 1..self.num_verts {
                        if b != v
                            && b != a
                            && (self.get_edge(v, b) == col)
                            && (self.get_edge(a, b) == col)
                        {
                            count += 1;
                        }
                    }
                }
            }
            ans.push(count);
        }
        ans
    }

    //counts red k4's that each vertex is a part of
    pub fn label_k4s(&self, col: bool) -> Vec<u32> {
        let mut ans = Vec::with_capacity(self.num_verts);

        for v in 0..self.num_verts {
            let mut count = 0u32;
            for a in 0..self.num_verts {
                if a != v && (self.get_edge(v, a) == col) {
                    for b in a + 1..self.num_verts {
                        if b != v
                            && b != a
                            && (self.get_edge(v, b) == col)
                            && (self.get_edge(a, b) == col)
                        {
                            for c in b + 1..self.num_verts {
                                if v != c
                                    && a != c
                                    && b != c
                                    && (self.get_edge(v, c) == col)
                                    && (self.get_edge(a, c) == col)
                                    && (self.get_edge(b, c) == col)
                                {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
            ans.push(count);
        }
        ans
    }

    //counts red k4's that each vertex is a part of
    pub fn label_k5s(&self, col: bool) -> Vec<u32> {
        let mut ans = Vec::with_capacity(self.num_verts);

        for v in 0..self.num_verts {
            let mut count = 0u32;
            for a in 0..self.num_verts {
                if a != v && (self.get_edge(v, a) == col) {
                    for b in a + 1..self.num_verts {
                        if b != v
                            && b != a
                            && (self.get_edge(v, b) == col)
                            && (self.get_edge(a, b) == col)
                        {
                            for c in b + 1..self.num_verts {
                                if v != c
                                    && a != c
                                    && b != c
                                    && (self.get_edge(v, c) == col)
                                    && (self.get_edge(a, c) == col)
                                    && (self.get_edge(b, c) == col)
                                {
                                    for d in c + 1..self.num_verts {
                                        if v != d
                                            && d != a
                                            && d != b
                                            && d != c
                                            && (self.get_edge(v, d) == col)
                                            && (self.get_edge(a, d) == col)
                                            && (self.get_edge(b, d) == col)
                                            && (self.get_edge(c, d) == col)
                                        {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ans.push(count);
        }
        ans
    }

    fn label_polygons(&self) -> Vec<u32> {
        if self.num_verts < 3 {
            return vec!(0; self.num_verts);
        }
        let mut out = Vec::new();
        use std::collections::VecDeque;
        for root in 0..self.num_verts {
            let mut loops = vec![0; self.num_verts];
            for left in 0..self.num_verts - 1 {
                if left == root || !self.get_edge(root, left) {
                    continue;
                }
                for right in left + 1..self.num_verts {
                    if right == root ||  !self.get_edge(root, right) {
                        continue;
                    }
                    //left and right are antennae, we are counting the shortest
                    //loop that connects them, if it exists

                    //to do this we do a breadth first search looking for right, starting from left
                    let mut length = 2;
                    let mut visited = vec![false; self.num_verts];
                    visited[root] = true;
                    visited[left] = true;
                    visited[right] = true;
                    let mut to_check = VecDeque::with_capacity(self.num_verts);
                    to_check.push_back((left, 1));
                    'a: loop {
                        match to_check.pop_front() {
                            Some((current, dist)) => {
                                if self.get_edge(current, right) {
                                    //+2 for this edge plus edge from right to root
                                    length = dist + 2;
                                    break 'a;
                                }
                                //no loop yet
                                for i in 0..self.num_verts {
                                    if visited[i] || i == current || !self.get_edge(current, i) {
                                        continue;
                                    }
                                    to_check.push_back((i, dist + 1));
                                    visited[i] = true;
                                }
                            }
                            None => break 'a,
                        }
                    }
                    loops[length - 2] += 1;
                }
            }

            let mut hasher = DefaultHasher::new();
            hasher.write(&loops);
            out.push(hasher.finish() as u32);
        }

        out
    }

    pub fn label(&self, buf: &mut Vec<Vec<(u32, u32)>>) -> Vec<u32> {
        if self.num_verts < 2 {
            return vec![0];
        }
        let mut d: Vec<u32> = Vec::with_capacity(self.num_verts);
        for i in 0..self.num_verts {
            let mut k = 0;
            for j in 0..self.num_verts {
                if i != j && self.get_edge(i as usize, j as usize) {
                    k += 1;
                }
            }
            d.push(k);
        }
        let p = self.label_polygons();
        //let t = self.label_k3s(true);
        //let q = self.label_k4s(true);
        //let q2 = self.label_k5s(true);
        //let q2 = vec![0; self.num_verts];
        for (l, pv) in d.iter_mut().zip(p) {
            //.zip(q).zip(q2) {
            let mut hasher = DefaultHasher::new();
            hasher.write_u32(*l);
            hasher.write_u32(pv);
            //hasher.write_u32(qv);
            //hasher.write_u32(q2v);
            *l = hasher.finish() as u32;
        }
        self.convolute(&mut d, buf);
        //self.convolute(&mut d, buf);
        //self.convolute(&mut d, buf);

        d
    }

    // This function takes an initial labeling, and produces a potentially better one(in place)
    fn convolute(&self, labels: &mut [u32], buf: &mut Vec<Vec<(u32, u32)>>) {
        for sub_buf in buf.iter_mut() {
            sub_buf.clear();
        }
        //kruskals for each vertex
        for root in 0..self.num_verts {
            let current_tree = &mut buf[root];
            let mut visited = vec![false; self.num_verts];
            visited[root] = true;
            let mut curr_dist = 0u32;
            loop {
                curr_dist += 1;
                let prev_visited = visited.clone();
                let mut to_break = true;
                for (i, vi) in prev_visited.iter().enumerate() {
                    if *vi {
                        for (j, vj) in visited.iter_mut().enumerate() {
                            if i != j && !*vj {
                                if self.get_edge(i, j) {
                                    *vj = true;
                                    to_break = false;
                                    current_tree.push((curr_dist, labels[j]));
                                }
                            }
                        }
                    }
                }
                if to_break {
                    break;
                }
            }
        }
        for i in 0..self.num_verts() {
            let mut hasher = DefaultHasher::new();
            hasher.write_u32(labels[i]); //previous label, to guarantee no worse labeling
            buf[i].sort();
            buf[i].hash(&mut hasher);
            labels[i] = hasher.finish() as u32
        }
    }

    pub fn to_string_append(&self, buf: &mut String) {
        base64::encode_config_buf(&self.edges.vec, base64::STANDARD_NO_PAD, buf);
    }

    // base 64 encoding of bits of edges
    pub fn to_string(&self) -> String {
        base64::encode(&self.edges.vec)
    }

    // decode base 64, size hint needed for space efficiency
    pub fn from_str(s: &str, num_verts: usize) -> Result<Graph, Box<dyn std::error::Error>> {
        let bits = base64::decode(s)?;
        Ok(Graph {
            num_verts,
            edges: BitVec {
                len: (num_verts as u16) * (num_verts - 1) as u16 / 2,
                vec: bits,
            },
        })
    }
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
