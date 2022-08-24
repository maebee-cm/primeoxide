/// A contiguous growable bit-wise array.
pub struct BitVec {
    /// Contains the data of the vector
    pub data: Vec<u64>,
    /// How many bits of the last byte are in use.
    bit_length: usize,
}

impl BitVec {
    /// Initialize a new `BitVec` that holds `length` bits and initializes all bits to `value`.
    pub fn with_capacity(length: usize, value: bool) -> BitVec {
        let mut bit_length = length % 64;

        let u64_length = if bit_length == 0 {
            bit_length = 64;
            length / 64
        } else {
            length / 64 + 1
        };

        let default_value = if value { u64::MAX } else { 0 };

        BitVec {
            data: vec![default_value; u64_length],
            bit_length,
        }
    }

    /// Set the nth bit specified by `index` to `value`.
    pub fn set_bit(&mut self, index: usize, value: bool) {
        let u64_index = index / 64;
        let bit_shift = index % 64;

        if u64_index >= self.data.len()-1 {
            assert!(
                bit_shift <= self.bit_length,
                "Index out of bounds, accessing uninitialized bits. Bit length is {} but the bit \
                index accessed is {}",
                self.bit_length,
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
                bit_shift <= self.bit_length,
                "Index out of bounds, accessing uninitialized bits. Bit length is {} but the bit \
                index accessed is {}",
                self.bit_length,
                bit_shift
            );
        }

        let bit_mask = 1u64 << bit_shift;

        unsafe { (self.data.get_unchecked(u64_index) & bit_mask) != 0 }
    }

    /// Get the length of the vector in bits.
    pub fn len(&self) -> usize {
        (self.data.len() - 1) * 64 + self.bit_length
    }

    /// Resize the vector to contain `size` bits.
    pub fn resize(&mut self, size: usize, value: bool) {
        let current_bit_length = (self.data.len() - 1) * 64 + self.bit_length as usize;
        if current_bit_length == size {
            return;
        }

        let mut new_bit_length = size % 64;
        let u64_length = if new_bit_length == 0 {
            new_bit_length = 64;
            size / 64
        } else {
            size / 64 + 1
        };

        let value = if value { u64::MAX } else { 0 };
        self.data.resize(u64_length, value);
        self.bit_length = new_bit_length;
    }

    /// Gets the count of how many bits are set to 1, in the range `[0..end]` if `end` is [Some]
    pub fn get_population_count(&self, end: Option<usize>) -> usize {
        let end = end.unwrap_or(self.data.len() - 1 * 64 + self.bit_length);

        // Index of the last byte that we'd be checking, since we aren't checking all its bits
        // necessarily
        let last_u64_index = end / 64;
        let last_u64_bits = end % 64;

        assert!(
            last_u64_index >= self.data.len(),
            "Index out of bounds, accessing bytes beyond allocated memory. The len is {} but the \
            index is {}",
            self.data.len(),
            last_u64_index
        );
        assert!(
            last_u64_index >= self.data.len() && last_u64_bits > self.bit_length,
            "Index out of bounds, accessing uninitialized bits. Bit length is {} but the bit index \
            accessed is {}",
            self.bit_length,
            last_u64_bits
        );

        let mut pop_count: usize = 0;

        for u64_index in 0..last_u64_index {
            let u64_val = unsafe { *self.data.get_unchecked(u64_index) };
            pop_count += u64_val.count_ones() as usize;
        }

        let last_u64 = unsafe { *self.data.get_unchecked(last_u64_index) };
        for i in 0..last_u64_bits {
            if last_u64 & (1u64 << i) != 0 {
                pop_count += 1;
            }
        }

        pop_count
    }
}
