/// A contiguous growable bit-wise array.
pub struct BitVec {
    /// Contains the data of the vector
    pub data: Vec<u64>,
    /// How many bits of the last byte are in use.
    last_byte_bits: usize,
}

impl BitVec {
    /// Initialize a new `BitVec` that holds `length` bits and initializes all bits to `value`.
    pub fn with_capacity(length: usize, value: bool) -> BitVec {
        let mut last_byte_bits = length % 64;

        let u64_length = if last_byte_bits == 0 {
            last_byte_bits = 64;
            length / 64
        } else {
            length / 64 + 1
        };

        let default_value = if value { u64::MAX } else { 0 };

        BitVec {
            data: vec![default_value; u64_length],
            last_byte_bits,
        }
    }

    /// Set the nth bit specified by `index` to `value`.
    pub fn set_bit(&mut self, index: usize, value: bool) {
        let u64_index = index / 64;
        let bit_shift = index % 64;

        if u64_index >= self.data.len()-1 {
            assert!(
                bit_shift <= self.last_byte_bits,
                "Index out of bounds, accessing uninitialized bits. Bit length is {} but the bit \
                index accessed is {}",
                self.last_byte_bits,
                bit_shift
            );
        }

        let bit_mask = if value {
            1u64 << (bit_shift)
        } else {
            !(1u64 << (bit_shift))
        };

        unsafe {
            *self.data.get_unchecked_mut(u64_index) &= bit_mask;
        }
    }

    /// Get the value of the nth bit specified by `index`.
    pub fn get_bit(&self, index: usize) -> bool {
        let u64_index = index / 64;
        let bit_shift = index % 64;

        if u64_index >= self.data.len() {
            assert!(
                bit_shift <= self.last_byte_bits,
                "Index out of bounds, accessing uninitialized bits. Bit length is {} but the bit \
                index accessed is {}",
                self.last_byte_bits,
                bit_shift
            );
        }

        let bit_mask = 1u64 << bit_shift;

        unsafe { (self.data.get_unchecked(u64_index) & bit_mask) != 0 }
    }

    /// Get the length of the vector in bits.
    pub fn len(&self) -> usize {
        (self.data.len() - 1) * 64 + self.last_byte_bits
    }

    /// Resize the vector to contain `size` bits.
    pub fn resize(&mut self, size: usize, value: bool) {
        let current_last_byte_bits = (self.data.len() - 1) * 64 + self.last_byte_bits as usize;
        if current_last_byte_bits == size {
            return;
        }

        let mut new_last_byte_bits = size % 64;
        let u64_length = if new_last_byte_bits == 0 {
            new_last_byte_bits = 64;
            size / 64
        } else {
            size / 64 + 1
        };

        let value = if value { u64::MAX } else { 0 };
        self.data.resize(u64_length, value);
        self.last_byte_bits = new_last_byte_bits;
    }

    /// Gets the count of how many bits are set to 1 from `[start..self.len()]`. If `start` is
    /// [None] then the range is `[0..self.len]`.
    pub fn get_population_count(&self, start: Option<usize>) -> u64 {
        let start = start.unwrap_or(0);
        let mut first_u64_index = start / 64;
        let first_byte_bits = start % 64;

        // Index of the last byte we're checking with count_ones(). If the self.last_byte_bits is
        // not exactly 64, then we have to use a slower method of counting.
        let last_u64_index = if self.last_byte_bits == 64 {
            self.data.len()
        } else {
            self.data.len() - 1
        };

        let mut pop_count: u64 = 0;

        // we have to round to the closest u64
        if first_byte_bits != 64 {
            let first_u64 = unsafe { *self.data.get_unchecked(first_u64_index) };
            for i in first_byte_bits..64 {
                if first_u64 & (1u64 << i) != 0 {
                    pop_count += 1;
                }
            }

            first_u64_index += 1;
        }

        for u64_index in first_u64_index..last_u64_index {
            let u64_val = unsafe { *self.data.get_unchecked(u64_index) };
            pop_count += u64_val.count_ones() as u64;
        }

        if self.last_byte_bits != 64 {
            let last_u64 = unsafe { *self.data.get_unchecked(last_u64_index) };
            for i in 0..self.last_byte_bits {
                if last_u64 & (1u64 << i) != 0 {
                    pop_count += 1;
                }
            }
        }

        pop_count
    }
}
