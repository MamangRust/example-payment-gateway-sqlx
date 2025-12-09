use async_trait::async_trait;
use std::sync::Arc;

pub type DynRateLimitMiddleware = Arc<dyn RateLimitMiddlewareTrait + Send + Sync>;

#[async_trait]
pub trait RateLimitMiddlewareTrait {
    async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: u32,
        window_seconds: u32,
    ) -> (bool, u32);
    async fn get_remaining(&self, key: &str, max_requests: u32) -> u32;
    async fn reset_limit(&self, key: &str) -> bool;
}
