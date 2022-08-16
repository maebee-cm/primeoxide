/// A contiguous growable bit-wise array.
pub struct BitVec {
    /// Contains the data of the vector
    pub data: Vec<u64>,
    /// How many bits of the last byte are in use.
    bit_length: u8
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            data: Vec::new(),
            bit_length: 0
        }
    }

    pub fn with_capacity(length: usize) -> BitVec {
        let bit_length = (length%64) as u8;

        let byte_length = if bit_length == 0 {
            length/64
        }
        else {
            length/64+1
        };

        let data = Vec::with_capacity(byte_length);

        BitVec {
            data,
            bit_length
        }
    }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        let byte_index = index/64;
        let bit_shift = if value {
            1u64 << (index%64)
        }
        else {
            !(1u64 << (index%64))
        };

        self.data[byte_index] &= bit_shift;
    }
}