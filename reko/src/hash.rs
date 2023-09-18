pub struct Hasher {
    data: [i32; 92],
}

impl Hasher {
    pub fn new() -> Self {
        Self { data: [0; 92] }
    }
    pub fn push(&mut self, stat: i32, value: i32) {
        self.data[stat as usize] += value;
    }
    pub fn finalize(&mut self) -> u64 {
        let mut hash: u64 = 0;
        for i in 0..64 {
            if self.data[i] > self.data[i + 1] {
                hash += 1 << (63 - i);
            }
        }
        hash
    }
}
