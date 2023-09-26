use structs::Stat;

pub struct Hasher {
    data: [i32; 65],
}

impl Hasher {
    pub fn new() -> Self {
        Self { data: [0; 65] }
    }
    pub fn push(&mut self, stat: Stat, value: i32) {
        if let Some(pos) = stat.hash_pos() {
            self.data[pos] += value;
        }
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
