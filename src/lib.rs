extern crate indicatif;
extern crate permutohedron;
extern crate rayon;
extern crate base64;

mod bitvec;
pub mod structures;

//#[cfg_attr(vec_graph, path = "graph_vec.rs")]
//#[cfg_attr(bv_graph, path = "graph_bv.rs")]
#[path = "graph_bv.rs"]
pub mod graph;

pub mod graph_like;

static COMPLEXITIES: &'static [u64] = &[
    1,  // - - - - - - - - - -  0
    1,  // - - - - - - - - - -  1
    2,  // - - - - - - - - - -  2
    6,  // - - - - - - - - - -  3
    24, // - - - - - - - - - -  4
    120,  // - - - - - - - - -  5
    720,  // - - - - - - - - -  6
    5_040,  // - - - - - - - -  7
    40_320, // - - - - - - - -  8
    362_880,    // - - - - - -  9
    3_628_800,  // - - - - - - 10
    39_916_800, // - - - - - - 11
    479_001_600,    // - - - - 12
    6_227_020_800,  // - - - - 13
    87_178_291_200, // - - - - 14
    1_307_674_368_000,  // - - 15
];


pub mod util {
    pub fn factorial(num: &usize) -> u64 {
        let mut i = 1;
        let mut c = num.clone() as u64;
        while c > 1 {
            i *= c;
            c -= 1;
        }
        i
    }

    pub fn dec_to_factorial(n: usize, dig: usize, out: &mut Vec<usize>) {
        let mut num = n;
        out.clear();
        for i in 0..dig {
            out.push(num % (i + 1));
            num /= i + 1;
        }
        out.reverse();
    }
    pub fn permute(
        list: &Vec<usize>,
        perm: &Vec<usize>,
        perm_scratch: &mut Vec<usize>,
        out: &mut Vec<usize>,
    ) {
        out.clear();
        perm_scratch.clone_from(list);
        for i in 0..list.len() {
            out.push(perm_scratch.remove(perm[i]));
        }
    }
}
