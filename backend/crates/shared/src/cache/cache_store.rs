use chrono::Duration;
use deadpool_redis::{Connection, Pool};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tracing::{debug, error, warn};

#[derive(Clone)]
pub struct CacheStore {
    redis_pool: Arc<Pool>,
}

impl CacheStore {
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

    pub async fn get_from_cache<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let mut conn = self.get_conn().await?;
        let result: redis::RedisResult<Option<String>> =
            redis::cmd("GET").arg(key).query_async(&mut conn).await;

        match result {
            Ok(Some(data)) => match serde_json::from_str::<T>(&data) {
                Ok(parsed) => Some(parsed),
                Err(e) => {
                    error!(
                        "Failed to deserialize cached value for key '{}': {:?}",
                        key, e
                    );
                    None
                }
            },
            Ok(None) => {
                warn!("Cache miss for key: {key}");
                None
            }
            Err(e) => {
                error!("Redis get error for key '{}': {:?}", key, e);
                None
            }
        }
    }

    pub async fn set_to_cache<T>(&self, key: &str, data: &T, expiration: Duration)
    where
        T: Serialize,
    {
        let json_data = match serde_json::to_string(data) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize data for key '{}': {:?}", key, e);
                return;
            }
        };

        if let Some(mut conn) = self.get_conn().await {
            let result: redis::RedisResult<()> = redis::pipe()
                .cmd("SET")
                .arg(key)
                .arg(&json_data)
                .ignore()
                .cmd("EXPIRE")
                .arg(key)
                .arg(expiration.num_seconds() as usize)
                .query_async(&mut conn)
                .await;

            match result {
                Ok(_) => debug!("Cached key '{}' with TTL {:?}", key, expiration),
                Err(e) => error!("Failed to set cache key '{}': {:?}", key, e),
            }
        }
    }

    pub async fn delete_from_cache(&self, key: &str) {
        if let Some(mut conn) = self.get_conn().await
            && let Err(e) = redis::cmd("DEL")
                .arg(key)
                .query_async::<()>(&mut conn)
                .await
        {
            error!("Failed to delete key '{}': {:?}", key, e);
        }
    }
}
