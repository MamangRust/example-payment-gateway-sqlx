use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct GatewayCircuitBreaker {
    failure_count: AtomicU64,
    success_count: AtomicU64,
    is_open: AtomicBool,
    last_failure_time: Arc<RwLock<Instant>>,
    threshold: u64,
    timeout: Duration,
    half_open_max_requests: u64,
}

impl GatewayCircuitBreaker {
    pub fn new(threshold: u64, timeout_secs: u64) -> Self {
        Self {
            failure_count: AtomicU64::new(0),
            success_count: AtomicU64::new(0),
            is_open: AtomicBool::new(false),
            last_failure_time: Arc::new(RwLock::new(Instant::now())),
            threshold,
            timeout: Duration::from_secs(timeout_secs),
            half_open_max_requests: 5,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open.load(Ordering::SeqCst)
    }

    pub fn get_failure_count(&self) -> u64 {
        self.failure_count.load(Ordering::SeqCst)
    }

    pub fn get_success_count(&self) -> u64 {
        self.success_count.load(Ordering::SeqCst)
    }

    pub async fn should_allow_request(&self) -> bool {
        if !self.is_open.load(Ordering::SeqCst) {
            return true;
        }

        let last_time = self.last_failure_time.read().await;
        if last_time.elapsed() > self.timeout {
            info!("⏱️  Circuit breaker timeout passed, entering half-open state");
            drop(last_time);

            let success = self.success_count.load(Ordering::SeqCst);
            if success < self.half_open_max_requests {
                return true;
            }

            if success >= self.half_open_max_requests {
                self.close_circuit();
                return true;
            }

            false
        } else {
            false
        }
    }

    pub fn record_success(&self) {
        let success = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;

        if self.is_open.load(Ordering::SeqCst) && success >= self.half_open_max_requests {
            self.close_circuit();
        }

        self.failure_count.store(0, Ordering::SeqCst);
    }

    pub fn record_failure(&self) {
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;

        if count >= self.threshold && !self.is_open.load(Ordering::SeqCst) {
            self.open_circuit();
        }
    }

    fn open_circuit(&self) {
        warn!(
            "🔴 Circuit breaker opened after {} failures",
            self.get_failure_count()
        );
        self.is_open.store(true, Ordering::SeqCst);
        self.success_count.store(0, Ordering::SeqCst);

        tokio::spawn({
            let last_time = self.last_failure_time.clone();
            async move {
                let mut time = last_time.write().await;
                *time = Instant::now();
            }
        });
    }

    fn close_circuit(&self) {
        info!("🟢 Circuit breaker closed - service recovered");
        self.is_open.store(false, Ordering::SeqCst);
        self.failure_count.store(0, Ordering::SeqCst);
        self.success_count.store(0, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.close_circuit();
    }
}

impl Default for GatewayCircuitBreaker {
    fn default() -> Self {
        Self::new(200, 30)
    }
}
