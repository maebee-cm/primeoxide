pub struct Wheel<'a> {
    /// A list of increments to loop through
    inc_list: &'a [u64],
    /// What index of the vector are we at
    vec_idx: usize,
}

impl<'a> Wheel<'a> {
    pub fn new(inc_list: &'a [u64]) -> Wheel {
        Wheel {
            inc_list,
            vec_idx: 0,
        }
    }

    pub fn next_inc(&mut self) -> u64 {
        let ret = unsafe { *self.inc_list.get_unchecked(self.vec_idx) };

        self.vec_idx += 1;
        if self.vec_idx == self.inc_list.len() {
            self.vec_idx = 0;
        }

        ret
    }

    pub fn sync(&mut self, other: &Wheel) {
        self.vec_idx = other.vec_idx;
    }

    pub fn reset(&mut self) {
        self.vec_idx = 0;
    }
}

impl<'a> Clone for Wheel<'a> {
    fn clone(&self) -> Self {
        Wheel {
            inc_list: self.inc_list,
            vec_idx: self.vec_idx,
        }
    }
}
