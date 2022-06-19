const ARRAY_SIZE_IN_BITS: usize = 8 * 4;
const TWO_TO_THE_SEVENTH: u8 = 2u8.pow(7);

pub struct BitArray {
    array: [u8; ARRAY_SIZE_IN_BITS / 8],
}

impl BitArray {
    pub fn new() -> BitArray {
        BitArray {
            array: [0; ARRAY_SIZE_IN_BITS / 8],
        }
    }

    pub fn set_bit(&mut self, i: usize) {
        self.array[i / 8] = self.array[i / 8] | (TWO_TO_THE_SEVENTH >> i % 8);
    }

    pub fn get_bit(&self, i: usize) -> bool {
        self.array[i / 8] & (TWO_TO_THE_SEVENTH >> i % 8) != 0u8
    }

    pub fn set_bit_from_u64(&mut self, i: u64) {
        self.set_bit(BitArray::bit_index_from_u64(i))
    }

    pub fn get_bit_from_u64(&mut self, i: u64) -> bool {
        self.get_bit(BitArray::bit_index_from_u64(i))
    }

    fn bit_index_from_u64(i: u64) -> usize {
        // The max value of f64 is always bigger than the max usize and u64, so the conversion of
        // the integers to floats are safe. The final value is guaranteed to be less than the array
        // size in bits, which is a usize, so it is safe to convert back to a usize.
        ((i as f64 / u64::MAX as f64) * (ARRAY_SIZE_IN_BITS - 1) as f64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_array_set_from_u64() {
        let mut ba = BitArray::new();

        ba.set_bit_from_u64(0);
        assert_eq!(ba.get_bit(0), true);
        assert_eq!(ba.get_bit_from_u64(0), true);

        ba.set_bit_from_u64(u64::MAX);
        assert_eq!(ba.get_bit(ARRAY_SIZE_IN_BITS - 1), true);
        assert_eq!(ba.get_bit_from_u64(u64::MAX), true);

        let test_u64 = u64::MAX / 2;
        ba.set_bit_from_u64(test_u64);
        assert_eq!(ba.get_bit_from_u64(test_u64), true);
        let bit_index = BitArray::bit_index_from_u64(test_u64);
        assert_eq!(ba.get_bit(bit_index), true);
        assert_eq!(ba.get_bit(bit_index + 1), false);
        assert_eq!(ba.get_bit(bit_index - 1), false);
    }

    #[test]
    fn bit_index_from_u64() {
        assert_eq!(BitArray::bit_index_from_u64(0), 0);
        assert_eq!(
            BitArray::bit_index_from_u64(u64::MAX),
            ARRAY_SIZE_IN_BITS - 1
        );
    }

    #[test]
    fn bit_array() {
        let mut ba = BitArray::new();

        ba.set_bit(ARRAY_SIZE_IN_BITS - 1);
        assert_eq!(ba.array[ARRAY_SIZE_IN_BITS / 8 - 1], 1u8);
        ba.set_bit(ARRAY_SIZE_IN_BITS - 2);
        assert_eq!(ba.array[ARRAY_SIZE_IN_BITS / 8 - 1], 3u8);
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
