use bit_vec::BitVec;

#[derive(Debug)]
pub struct BloomFilter {
    pub buf: BitVec
}

impl BloomFilter {
    pub fn new(m: usize) -> BloomFilter {
        let buf = BitVec::from_elem(m, false);

        BloomFilter {
            buf,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;

    #[test]
    fn it_works() {
        let bloom = BloomFilter::new(4);
        println!("bloom: {:?}", bloom);
        assert_eq!(bloom.buf.len(), 4);
    }
}
