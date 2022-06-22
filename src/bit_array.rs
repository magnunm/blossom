const TWO_TO_THE_SEVENTH: u8 = 2u8.pow(7);

/// The bit array underlying a Bloom filter.
pub struct BitArray {
    array: Vec<u8>,
    size_in_bits: usize,
}

impl BitArray {
    pub fn new(size_in_bits: usize) -> BitArray {
        // Ensure we allocate enough bytes by rounding up to the nearest multiple of 8.
        let size_in_bytes = if size_in_bits % 8 == 0 {
            size_in_bits / 8
        } else {
            (size_in_bits + 8 - size_in_bits % 8) / 8
        };

        BitArray {
            array: vec![0; size_in_bytes],
            size_in_bits,
        }
    }

    /// Set a bit to 1 based on the output of a hash function.
    pub fn set_bit_from_u64(&mut self, i: u64) {
        self.set_bit(self.bit_index_from_u64(i))
    }

    /// Get the value of a bit based on the output of a hash function.
    pub fn get_bit_from_u64(&self, i: u64) -> bool {
        self.get_bit(self.bit_index_from_u64(i))
    }

    fn set_bit(&mut self, i: usize) {
        self.array[i / 8] = self.array[i / 8] | (TWO_TO_THE_SEVENTH >> i % 8);
    }

    fn get_bit(&self, i: usize) -> bool {
        self.array[i / 8] & (TWO_TO_THE_SEVENTH >> i % 8) != 0u8
    }

    fn bit_index_from_u64(&self, i: u64) -> usize {
        // The max value of f64 is always bigger than the max usize and u64, so the conversion of
        // the integers to floats are safe. The final value is guaranteed to be less than the array
        // size in bits, which is a usize, so it is safe to convert back to a usize.
        ((i as f64 / u64::MAX as f64) * (self.size_in_bits - 1) as f64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_array_set_from_u64() {
        let size = 8 * 4 + 3;
        let mut ba = BitArray::new(size);

        ba.set_bit_from_u64(0);
        assert_eq!(ba.get_bit(0), true);
        assert_eq!(ba.get_bit_from_u64(0), true);

        ba.set_bit_from_u64(u64::MAX);
        assert_eq!(ba.get_bit(size - 1), true);
        assert_eq!(ba.get_bit_from_u64(u64::MAX), true);

        let test_u64 = u64::MAX / 2;
        ba.set_bit_from_u64(test_u64);
        assert_eq!(ba.get_bit_from_u64(test_u64), true);
        let bit_index = ba.bit_index_from_u64(test_u64);
        assert_eq!(ba.get_bit(bit_index), true);
        assert_eq!(ba.get_bit(bit_index + 1), false);
        assert_eq!(ba.get_bit(bit_index - 1), false);
    }

    #[test]
    fn bit_index_from_u64() {
        let size = 8 * 4 + 1;
        let ba = BitArray::new(size);

        assert_eq!(ba.bit_index_from_u64(0), 0);
        assert_eq!(ba.bit_index_from_u64(u64::MAX), size - 1);
    }

    #[test]
    fn bit_array() {
        let size = 8 * 4;
        let mut ba = BitArray::new(size);

        ba.set_bit(size - 1);
        assert_eq!(ba.array[size / 8 - 1], 1u8);
        ba.set_bit(size - 2);
        assert_eq!(ba.array[size / 8 - 1], 3u8);
        ba.set_bit(0);
        assert_eq!(ba.array[0], 2u8.pow(7));
        ba.set_bit(1);
        assert_eq!(ba.array[0], 2u8.pow(7) + 2u8.pow(6));
        ba.set_bit(1);
        assert_eq!(ba.array[0], 2u8.pow(7) + 2u8.pow(6));

        assert_eq!(ba.get_bit(0), true);
        assert_eq!(ba.get_bit(1), true);
        assert_eq!(ba.get_bit(2), false);
        assert_eq!(ba.get_bit(3), false);
    }
}
