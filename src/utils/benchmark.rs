use std::time::Instant;

pub struct Time {
    name: String,
    start: Instant,
    last: Instant,
    is_first: bool,
}

impl Time {
    pub fn start(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: Instant::now(),
            last: Instant::now(),
            is_first: true,
        }
    }
    pub fn millis(&mut self, s: String) {
        println!(" {} - {} ms", s, self.last.elapsed().as_millis());
        if self.is_first {
            println!("\n\n{}\n-----------------\n", self.name);
            self.is_first = false;
        }
        self.last = Instant::now();
    }
    pub fn micros(&mut self, s: String) {
        println!(" {} - {} μs", s, self.last.elapsed().as_micros());
        if self.is_first {
            println!("\n\n{}\n-----------------\n", self.name);
            self.is_first = false;
        }
        self.last = Instant::now();
    }
}
