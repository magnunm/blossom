use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod bit_array;

use bit_array::BitArray;

/// A Bloom filter.
///
/// A Bloom filter is a probabilistic data structure that can be used to represent a set. When
/// testing for membership the Bloom filter will either tell you that "the element is definitely
/// not in the set" or "the element is maybe in the set". The advantage of the Bloom filter over
/// set representations with a deterministic membership test it that it can often be much smaller
/// in size. The size of the bloom filter is for example independent of both the size of the
/// inserted elements and the number of elements inserted. At the same time both insertions and
/// membership tests have constant time complexity. Elements can not be removed from a Bloom
/// filter.
///
/// The probability of false positives can be reduced, at the cost of increasing the size of the
/// Bloom filter and slowing down insertions and membership tests. Use
/// [`with_false_positive_bound`] to construct a Bloom filter with an upper bound on the false
/// positive probability.
///
/// [`with_false_positive_bound`]: BloomFilter::with_false_positive_bound
pub struct BloomFilter {
    bit_array: BitArray,
    num_hash_functions: usize,
}

impl BloomFilter {
    /// Construct a Bloom filter.
    ///
    /// Explicitly giving the size in bits of the underlying bit array, and the number of different
    /// hash functions to use.
    pub fn new(size: usize, num_hash_functions: usize) -> BloomFilter {
        BloomFilter {
            bit_array: BitArray::new(size),
            num_hash_functions,
        }
    }

    /// Construct a Bloom filter with the given upper bound for the false positive probability.
    ///
    /// Where the upper bound is valid as long as no more than the given maximum number of elements
    /// are inserted into the bloom filter. Upper bound taken from
    /// https://en.wikipedia.org/wiki/Bloom_filter.
    pub fn with_false_positive_bound(
        false_positive_probability: f32,
        max_insertions: u32,
    ) -> BloomFilter {
        let multiplier = -false_positive_probability.ln() / 2f32.ln().powf(2.0);
        let size = (max_insertions as f32 * multiplier).ceil();

        if size > usize::MAX as f32 {
            panic!(
                concat!(
                    "The bit array size required to reach this false positive bound is ",
                    "larger than the maximum allowed: {}"
                ),
                usize::MAX
            );
        }

        // No need to check this conversion to `usize` since this value is strictly less than the
        // size computed above.
        let num_hash_functions = (-false_positive_probability.log2()).ceil() as usize;

        BloomFilter::new(size as usize, num_hash_functions)
    }

    /// Insert an element into the Bloom filter.
    pub fn insert<T: Hash>(&mut self, value: T) {
        for i in 0..self.num_hash_functions {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            value.hash(&mut hasher);
            self.bit_array.set_bit_from_u64(hasher.finish());
        }
    }

    /// Query for the given value in the Bloom filter.
    ///
    /// If this returns `false` the value is guaranteed to never have been inserted. When it
    /// returns `true` the value has either been inserted, or the bits that would have been set by
    /// inserting the element has by chance been by other insertions. The latter case is called a
    /// false positive.
    pub fn maybe_contains<T: Hash>(&self, value: T) -> bool {
        for i in 0..self.num_hash_functions {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            value.hash(&mut hasher);

            if !self.bit_array.get_bit_from_u64(hasher.finish()) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_query_single_element() {
        let mut bf = BloomFilter::new(8 * 4, 1);

        bf.insert(12);
        assert_eq!(bf.maybe_contains(12), true);
        assert_eq!(bf.maybe_contains(13), false);
        assert_eq!(bf.maybe_contains(14), false);

        let mut bf = BloomFilter::new(8 * 4, 10);

        bf.insert("foo");
        assert_eq!(bf.maybe_contains("foo"), true);
        assert_eq!(bf.maybe_contains("bar"), false);
        assert_eq!(bf.maybe_contains("baz"), false);
    }

    #[test]
    fn with_false_positive_bound() {
        let mut bf = BloomFilter::with_false_positive_bound(0.01, 1000);

        for i in 0..100 {
            bf.insert(i);
            assert_eq!(bf.maybe_contains(i), true);
        }

        // This could fail here due to the bloom filter giving a false positive, but the
        // probability is less than 1%.
        assert_eq!(bf.maybe_contains(100), false);
    }
}
