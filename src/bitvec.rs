//A very simple, unsafe, but fast bitvec implementation
//Definitely not complete, but well suited for purpose

#[derive(Debug, Default, Clone)]
pub struct BitVec {
    pub len: u16,
    pub vec: Vec<u8>,
}

impl BitVec {
    pub fn new() -> Self {
        Self::default()
    }
    
    //least significant bits are first for each u64
    pub fn push(&mut self, val: bool) {
        if (self.len/8) as usize >= self.vec.len() {
            self.vec.push(0);
        }
        if val {
            let word = (self.len/8) as usize;
            let bit = (7 - self.len%8);
            self.vec[word] |= 1 << bit;
        }
        self.len += 1;
    }

    //extremely unsafe, know what you're doing
    pub fn get(&self, index: usize) -> bool {
        let word = index/8;
        let bit = (7 - index%8);
        self.vec[word] & (1 << bit) != 0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pushing() {
        let mut bv = BitVec::new();
        assert!(bv.len == 0);

        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(true);
        bv.push(false);
        assert!(bv.len == 5);

        assert!(bv.get(0) == true);
        assert!(bv.get(1) == false);
        assert!(bv.get(2) == true);
        assert!(bv.get(3) == true);
        assert!(bv.get(4) == false);
    }
}
