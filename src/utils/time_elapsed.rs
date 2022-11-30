use std::time::Instant;

pub fn start<S: AsRef<str>>(name: S) -> TimeElapsed {
    TimeElapsed::init(name.as_ref())
}

fn get_unit_of_measurement(nanos: u128) -> &'static str {
    match nanos / 4000000 {
        0 => "μs",
        _ => match nanos / 15000000000 {
            0 => "ms",
            _ => match nanos /  300000000000 {
                0 => "s",
                _ => match  nanos / 540000000000 {
                    0 => "min",
                    _ => "hrs"
                }
            },
        },
    }
}

fn get_units_of_measurement(nanos: u128) -> [&'static str; 2] {
    match get_unit_of_measurement(nanos) {
        "μs" => ["μs", "ns"],
        "ms" => ["ms", "μs"],
        "s" => ["s", "ms"],
        "min" => ["min", "s"],
        "hrs" => ["hrs", "min"],
        _ => ["ns", "ns"]
    }
}

fn convert_to_unit_of_meas(nanos: u128, unit_of_meas: &str) -> u128 {
    match unit_of_meas {
        "μs" => nanos / 1000,
        "ms" => nanos / 1000000,
        "s" => nanos / 1000000000,
        "min" => nanos / 60000000000,
        "hrs" => nanos / 3600000000000,
        _ => nanos,
    }
}

fn convert_to_units_of_meas(nanos: u128, unit_of_meas: &str) -> [u128; 2] {
    match unit_of_meas {
        "μs" => [nanos / 1000, nanos],
        "ms" => [nanos / 1000000, nanos / 1000],
        "s" => [nanos / 1000000000, nanos / 1000000],
        "min" => [nanos / 60000000000, nanos / 1000000000],
        "hrs" => [nanos / 3600000000000, nanos / 60000000000],
        _ => [nanos, nanos],
    }
}

pub struct TimeElapsed {
    name: String,
    start_timestamp: Instant,
    last_timestamp: Instant,
}

impl TimeElapsed {
    fn init(name: &str) -> Self {
        println!("{} running...", name);
        Self {
            name: name.to_string(),
            start_timestamp: Instant::now(),
            last_timestamp: Instant::now(),
        }
    }

    fn print_message(&mut self, msg: &str, nanos: u128) -> &Self {
        let unit = get_unit_of_measurement(nanos);
        let time = convert_to_unit_of_meas(nanos, unit);
        println!(
            "({})\x1b[32m \x1b[1m{} \x1b[0m-> \x1b[93m\x1b[1m{} {} \x1b[0m",
            self.name, msg, time, unit
        );
        self
    }

    pub fn end(self) {
        let nanos = self.start_timestamp.elapsed().as_nanos();
        let units = get_units_of_measurement(nanos);
        let times = convert_to_units_of_meas(nanos, units[0]);
        println!(
            "\x1b[32m\x1b[1m{} \x1b[0mfinished in \x1b[93m\x1b[1m{} {} \x1b[0m({} {})",
            self.name, times[0], units[0], times[1], units[1],
        );
    }

    pub fn log<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        let nanos = self.last_timestamp.elapsed().as_nanos();
        self.print_message(msg.as_ref(), nanos);
        self
    }

    pub fn _log_from_start<S: AsRef<str>>(&mut self, msg: S) -> &mut Self {
        let nanos = self.start_timestamp.elapsed().as_nanos();
        self.print_message(msg.as_ref(), nanos);
        self
    }

    pub fn timestamp(&mut self) -> Instant {
        self.last_timestamp = Instant::now();
        self.last_timestamp
    }
}
