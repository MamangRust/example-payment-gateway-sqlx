use crate::abstract_trait::rate_limit::RateLimitMiddlewareTrait;
use async_trait::async_trait;
use deadpool_redis::{Connection, Pool};
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Clone)]
pub struct RateLimiter {
    redis_pool: Arc<Pool>,
}

impl RateLimiter {
    pub fn new(redis_pool: Pool) -> Self {
        Self {
            redis_pool: Arc::new(redis_pool),
        }
    }

    async fn get_conn(&self) -> Option<Connection> {
        match self.redis_pool.get().await {
            Ok(conn) => Some(conn),
            Err(e) => {
                error!("Failed to get Redis pooled connection: {:?}", e);
                None
            }
        }
    }
}

#[async_trait]
impl RateLimitMiddlewareTrait for RateLimiter {
    async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: u32,
        window_seconds: u32,
    ) -> (bool, u32) {
        let mut conn = match self.get_conn().await {
            Some(conn) => conn,
            None => return (false, 0),
        };

        let current: u32 = match deadpool_redis::redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
        {
            Ok(Some(val)) => val,
            _ => 0u32,
        };

        if current >= max_requests {
            debug!("Rate limit exceeded for key: {key}");
            return (false, current);
        }

        let _: () = deadpool_redis::redis::pipe()
            .atomic()
            .cmd("INCR")
            .arg(key)
            .ignore()
            .cmd("EXPIRE")
            .arg(key)
            .arg(window_seconds)
            .ignore()
            .query_async(&mut conn)
            .await
            .unwrap_or(());

        (true, current + 1)
    }

    async fn get_remaining(&self, key: &str, max_requests: u32) -> u32 {
        let mut conn = match self.get_conn().await {
            Some(conn) => conn,
            None => return 0,
        };

        let current: u32 = match deadpool_redis::redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
        {
            Ok(Some(val)) => val,
            _ => 0u32,
        };

        max_requests.saturating_sub(current)
    }

    async fn reset_limit(&self, key: &str) -> bool {
        if let Some(mut conn) = self.get_conn().await {
            deadpool_redis::redis::cmd("DEL")
                .arg(key)
                .query_async::<()>(&mut conn)
                .await
                .is_ok()
        } else {
            false
        }
    }
}
