use bit_vec::BitVec;

#[derive(Debug)]
pub struct BloomFilter {
    pub buf: BitVec
}

fn h1(key: usize) -> usize {
    return key % 8
}

fn h2(key: usize) -> usize {
    return (key * 2) % 8
}

impl BloomFilter {
    pub fn new(m: usize) -> BloomFilter {
        let buf = BitVec::from_elem(m, false);

        BloomFilter {
            buf,
        }
    }

    pub fn set(&mut self, key: usize) {
        let a = h1(key);
        let b = h2(key);
        self.buf.set(a, true);
        self.buf.set(b, true);
    }

    pub fn is_exit(&self, key: usize) -> bool {
        let h1_bit: bool;
        match self.buf.get(h1(key)) {
            Some(x) => h1_bit = x,
            None => return false
        }
        let h2_bit: bool;
        match self.buf.get(h2(key)) {
            Some(x) => h2_bit = x,
            None => return false
        }
        return h1_bit && h2_bit
    }
}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;

    #[test]
    fn test_set_bit() {
        let mut bloom = BloomFilter::new(16);
        bloom.set(1000);
        assert_eq!(bloom.is_exit(1000), true);
        assert_eq!(bloom.is_exit(1001), false);
    }

    #[test]
    fn test_false_positive() {
        let mut bloom = BloomFilter::new(16);
        bloom.set(1000);
        bloom.set(1001);
        bloom.set(1004);
        assert_eq!(bloom.is_exit(1005), false);
        assert_eq!(bloom.is_exit(1020), true);
    }
}
