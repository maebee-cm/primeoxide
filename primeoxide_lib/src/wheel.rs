pub struct Wheel {
    /// A list of increments to loop through
    inc_list: Vec<u64>,
    /// What index of the vector are we at
    vec_idx: usize,
}

impl Wheel {
    pub fn new(inc_list: Vec<u64>) -> Wheel {
        Wheel {
            inc_list,
            vec_idx: 0
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
}
