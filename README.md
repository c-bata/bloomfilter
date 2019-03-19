# bloomfilter

See https://en.wikipedia.org/wiki/Bloom_filter for more details.

Usage:

```rust
let mut bloom = BloomFilter::new(16);
bloom.set(1000);
assert_eq!(bloom.is_exit(1000), true);
assert_eq!(bloom.is_exit(1001), false);

bloom.set(1001);
bloom.set(1004);
assert_eq!(bloom.is_exit(1005), false);
assert_eq!(bloom.is_exit(1020), true);
```

**Special thanks:**

* https://speakerdeck.com/kakakakakku/bloom-filter (japanese)

**Next step:**

* Counting filter: https://dl.acm.org/citation.cfm?id=3035963
* Scalable bloom filter: https://haslab.uminho.pt/cbm/publications/scalable-bloom-filters
