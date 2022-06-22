use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod bit_array;

use bit_array::BitArray;

pub struct BloomFilter {
    bit_array: BitArray,
    num_hash_functions: usize,
}

impl BloomFilter {
    pub fn new(size: usize, num_hash_functions: usize) -> BloomFilter {
        BloomFilter {
            bit_array: BitArray::new(size),
            num_hash_functions,
        }
    }

    /// Construct a bloom filter with the given upper bound for the false positive probability.
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

        if size + 8.0 > usize::MAX as f32 {
            panic!(
                concat!(
                    "The bit array size required to reach this false positive bound is ",
                    "larger then the maximum allowed: {}"
                ),
                usize::MAX
            );
        }

        // The size must be a multiple of 8 since the bit array is a vector of bytes.
        let mut size_as_int = size as usize;
        size_as_int += if size_as_int % 8 == 0 {
            0
        } else {
            8 - size_as_int % 8
        };

        // No need to check this conversion since this value is strictly less than the size
        // computed above.
        let num_hash_functions = (-false_positive_probability.log2()).ceil() as usize;

        BloomFilter::new(size_as_int, num_hash_functions)
    }

    pub fn insert<T: Hash>(&mut self, value: T) {
        for i in 0..self.num_hash_functions {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            value.hash(&mut hasher);
            self.bit_array.set_bit_from_u64(hasher.finish());
        }
    }

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
