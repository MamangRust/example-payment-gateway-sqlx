use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{sync::RwLock, time::Instant};

pub struct LoadMonitor {
    request_count: AtomicU64,
    last_reset: RwLock<Instant>,
}

impl Default for LoadMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadMonitor {
    pub fn new() -> Self {
        Self {
            request_count: AtomicU64::new(0),
            last_reset: RwLock::new(Instant::now()),
        }
    }

    pub fn record_request(&self) {
        self.request_count.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn get_current_rps(&self) -> u64 {
        let count = self.request_count.swap(0, Ordering::SeqCst);

        let mut last = self.last_reset.write().await;
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();
        *last = now;

        if elapsed > 0.0 {
            (count as f64 / elapsed) as u64
        } else {
            0
        }
    }
}
