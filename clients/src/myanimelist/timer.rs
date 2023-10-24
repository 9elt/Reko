use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use util::{sleep, MAX_MAL_REQ_PER_SECOND};

const MIN_SLEEP: u16 = (1000.0 / MAX_MAL_REQ_PER_SECOND) as u16;

pub type WrappedTimer = Arc<Mutex<Timer>>;

#[derive(Clone)]
pub struct Timer(Instant);

impl Timer {
    pub fn new() -> WrappedTimer {
        Arc::from(Mutex::from(Self(Instant::now())))
    }
    pub fn sleep(&mut self) {
        let elapsed = Instant::now().duration_since(self.0).as_millis();

        if elapsed < MIN_SLEEP as u128 {
            sleep(MIN_SLEEP - elapsed as u16);
        }

        self.0 = Instant::now();
    }
}
