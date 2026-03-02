use crate::errors::CircuitBreakerError;
use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::Duration,
};
use tokio::{sync::RwLock, time::Instant};
use tracing::{info, warn};

pub struct CircuitBreaker {
    failure_count: AtomicU64,
    is_open: AtomicBool,
    last_failure_time: RwLock<Instant>,
    threshold: u64,
    timeout: Duration,
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self {
            failure_count: AtomicU64::new(0),
            is_open: AtomicBool::new(false),
            last_failure_time: RwLock::new(Instant::now()),
            threshold: 100,
            timeout: Duration::from_secs(60),
        }
    }

    pub async fn call_async<F, Fut, T, E>(&self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        if self.is_open.load(Ordering::Acquire) {
            let last = self.last_failure_time.read().await;
            if last.elapsed() < self.timeout {
                return Err(CircuitBreakerError::Open);
            }

            self.is_open.store(false, Ordering::Release);
            self.failure_count.store(0, Ordering::Relaxed);
        }

        match f().await {
            Ok(val) => {
                self.failure_count.store(0, Ordering::Relaxed);
                Ok(val)
            }
            Err(err) => {
                let count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                if count >= self.threshold {
                    warn!("Circuit breaker opened after {} failures", count);
                    self.is_open.store(true, Ordering::Release);
                    let mut last = self.last_failure_time.write().await;
                    *last = Instant::now();
                }
                Err(CircuitBreakerError::Inner(err))
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open.load(Ordering::SeqCst)
    }

    pub fn reset(&self) {
        self.is_open.store(false, Ordering::SeqCst);
        self.failure_count.store(0, Ordering::SeqCst);
        info!("🟢 Circuit breaker reset");
    }
}
