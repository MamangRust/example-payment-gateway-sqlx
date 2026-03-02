use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct GatewayRequestLimiter {
    pub semaphore: Arc<Semaphore>,
    pub max_concurrent: usize,
}

impl GatewayRequestLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            max_concurrent,
        }
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }
}

impl Default for GatewayRequestLimiter {
    fn default() -> Self {
        Self::new(2000)
    }
}
