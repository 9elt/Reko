use crate::util::MAX_MAL_REQ_PER_SECOND;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{sleep_until, Instant};

const MIN_INTERVAL: Duration = Duration::from_millis((1000.0 / MAX_MAL_REQ_PER_SECOND) as u64);

#[derive(Clone)]
pub struct Timer(Arc<Mutex<Instant>>);

impl Timer {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Instant::now() - MIN_INTERVAL)))
    }
    pub async fn wait(&self) {
        let mut last = self.0.lock().await;
        sleep_until(*last + MIN_INTERVAL).await;
        *last = Instant::now();
    }
}
