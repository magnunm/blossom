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
}
