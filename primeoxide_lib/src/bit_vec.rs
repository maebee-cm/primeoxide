/// A contiguous growable bit-wise array.
pub struct BitVec {
    /// Contains the data of the vector
    pub data: Vec<u64>,
    /// How many bits of the last byte are in use.
    bit_length: u8
}

impl BitVec {
    pub fn with_capacity(length: usize, default: bool) -> BitVec {
        let mut bit_length = (length%64) as u8;

        let byte_length = if bit_length == 0 {
            bit_length = 64;
            length/64
        }
        else {
            length/64+1
        };

        let default_value = if default { u64::MAX } else { 0 };

        BitVec {
            data: vec![default_value; byte_length],
            bit_length
        }
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        let u64_index = index/64;
        let bit_shift = (index%64) as u8;

        if u64_index == self.data.len()-1 && bit_shift > self.bit_length {
            panic!("Accessing uninitialized bits. Bit length: {}, bit accessed: {}",
                   self.bit_length, bit_shift);
        }

        let bit_mask = if value {
            1u64 << (bit_shift)
        }
        else {
            !(1u64 << (bit_shift))
        };

        unsafe {
            *self.data.get_unchecked_mut(u64_index) &= bit_mask;
        }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let u64_index = index/64;
        let bit_shift = (index%64) as u8;

        if u64_index == self.data.len()-1 && bit_shift > self.bit_length {
            panic!("Accessing uninitialized bits. Bit length: {}, bit accessed: {}",
                     self.bit_length, bit_shift);
        }

        let bit_mask = 1u64 << bit_shift;

        unsafe { (self.data.get_unchecked(u64_index) & bit_mask) != 0 }
    }
}