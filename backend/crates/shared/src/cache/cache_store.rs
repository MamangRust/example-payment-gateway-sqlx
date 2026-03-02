use anyhow::{Context, Result};
use chrono::Duration;
use deadpool_redis::{Connection, Pool};
use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

use crate::observability::{CacheMetrics, CacheOperation};

#[derive(Clone)]
pub struct CacheStore {
    redis_pool: Arc<Pool>,
    metrics: CacheMetrics,
}

impl CacheStore {
    pub fn new(redis_pool: Pool, metrics: CacheMetrics) -> Self {
        Self {
            redis_pool: Arc::new(redis_pool),
            metrics,
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
        let start = Instant::now();

        let mut conn = match self.get_conn().await {
            Some(c) => c,
            None => {
                self.metrics
                    .record_error(CacheOperation::Get, start.elapsed().as_secs_f64());
                return None;
            }
        };

        let result: redis::RedisResult<Option<String>> =
            redis::cmd("GET").arg(key).query_async(&mut conn).await;

        let duration = start.elapsed().as_secs_f64();

        match result {
            Ok(Some(data)) => match serde_json::from_str::<T>(&data) {
                Ok(parsed) => {
                    self.metrics.record_hit(CacheOperation::Get, duration);
                    debug!("Cache hit for key: {}", key);
                    Some(parsed)
                }
                Err(e) => {
                    error!(
                        "Failed to deserialize cached value for key '{}': {:?}",
                        key, e
                    );
                    self.metrics.record_error(CacheOperation::Get, duration);
                    None
                }
            },
            Ok(None) => {
                self.metrics.record_miss(CacheOperation::Get, duration);
                debug!("Cache miss for key: {}", key);
                None
            }
            Err(e) => {
                error!("Redis get error for key '{}': {:?}", key, e);
                self.metrics.record_error(CacheOperation::Get, duration);
                None
            }
        }
    }

    pub async fn set_to_cache<T>(&self, key: &str, data: &T, expiration: Duration)
    where
        T: Serialize,
    {
        let start = Instant::now();

        let json_data = match serde_json::to_string(data) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize data for key '{}': {:?}", key, e);
                self.metrics
                    .record_error(CacheOperation::Set, start.elapsed().as_secs_f64());
                return;
            }
        };

        let ttl = expiration.num_seconds();
        if ttl <= 0 {
            warn!(
                "Skipping cache set for key '{}' due to non-positive TTL",
                key
            );
            self.metrics
                .record_error(CacheOperation::Set, start.elapsed().as_secs_f64());
            return;
        }

        if let Some(mut conn) = self.get_conn().await {
            let result: redis::RedisResult<()> = conn.set_ex(key, json_data, ttl as u64).await;
            let duration = start.elapsed().as_secs_f64();

            match result {
                Ok(_) => {
                    self.metrics.record_success(CacheOperation::Set, duration);
                    debug!("Cached key '{}' with TTL {}s", key, ttl);
                }
                Err(e) => {
                    error!("Failed to set cache key '{}': {:?}", key, e);
                    self.metrics.record_error(CacheOperation::Set, duration);
                }
            }
        } else {
            self.metrics
                .record_error(CacheOperation::Set, start.elapsed().as_secs_f64());
        }
    }

    pub async fn delete_from_cache(&self, key: &str) {
        let start = Instant::now();

        if let Some(mut conn) = self.get_conn().await {
            let result = redis::cmd("DEL")
                .arg(key)
                .query_async::<()>(&mut conn)
                .await;

            let duration = start.elapsed().as_secs_f64();

            match result {
                Ok(_) => {
                    self.metrics
                        .record_success(CacheOperation::Delete, duration);
                    debug!("Deleted key: {}", key);
                }
                Err(e) => {
                    error!("Failed to delete key '{}': {:?}", key, e);
                    self.metrics.record_error(CacheOperation::Delete, duration);
                }
            }
        } else {
            self.metrics
                .record_error(CacheOperation::Delete, start.elapsed().as_secs_f64());
        }
    }

    pub async fn clear_expired(&self) -> Result<usize, String> {
        let start = Instant::now();
        info!("🧹 Clearing expired cache entries...");

        let mut conn = match self.get_conn().await {
            Some(c) => c,
            None => {
                self.metrics
                    .record_error(CacheOperation::Clear, start.elapsed().as_secs_f64());
                return Err("Failed to get Redis connection".to_string());
            }
        };

        let mut cursor: u64 = 0;
        let mut total_scanned = 0;
        let mut deleted = 0;

        loop {
            let result: redis::RedisResult<(u64, Vec<String>)> = redis::cmd("SCAN")
                .arg(cursor)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await;

            match result {
                Ok((new_cursor, keys)) => {
                    total_scanned += keys.len();

                    for key in keys {
                        let ttl_result: redis::RedisResult<i64> =
                            redis::cmd("TTL").arg(&key).query_async(&mut conn).await;

                        if let Ok(ttl) = ttl_result
                            && (ttl == -2 || ttl == 0)
                            && (redis::cmd("DEL")
                                .arg(&key)
                                .query_async::<()>(&mut conn)
                                .await)
                                .is_ok()
                        {
                            deleted += 1;
                        }
                    }

                    cursor = new_cursor;
                    if cursor == 0 {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to scan keys: {:?}", e);
                    self.metrics
                        .record_error(CacheOperation::Clear, start.elapsed().as_secs_f64());
                    return Err(format!("SCAN error: {:?}", e));
                }
            }
        }

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_success(CacheOperation::Clear, duration);

        info!(
            "✅ Cache cleanup completed. Scanned: {}, Deleted: {}",
            total_scanned, deleted
        );
        Ok(total_scanned)
    }

    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<usize> {
        let start = Instant::now();
        info!("🗑️  Invalidating cache pattern: {}", pattern);

        let mut conn = self.get_conn().await.ok_or_else(|| {
            self.metrics
                .record_error(CacheOperation::Invalidate, start.elapsed().as_secs_f64());
            anyhow::anyhow!("Failed to get Redis connection")
        })?;

        let mut cursor: u64 = 0;
        let mut deleted = 0;

        loop {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
                .context("Failed to execute SCAN command")?;

            if !keys.is_empty() {
                for key in keys {
                    match redis::cmd("DEL")
                        .arg(&key)
                        .query_async::<()>(&mut conn)
                        .await
                    {
                        Ok(_) => deleted += 1,
                        Err(e) => {
                            error!("Failed to delete key {}: {:?}", key, e);
                        }
                    }
                }
            }

            cursor = new_cursor;
            if cursor == 0 {
                break;
            }
        }

        let duration = start.elapsed().as_secs_f64();
        self.metrics
            .record_success(CacheOperation::Invalidate, duration);

        info!(
            "✅ Invalidated {} keys matching pattern: {}",
            deleted, pattern
        );

        Ok(deleted)
    }

    pub async fn get_stats(&self) -> Result<CacheStats, String> {
        let start = Instant::now();
        debug!("Fetching cache stats from Redis...");

        let mut conn = match self.get_conn().await {
            Some(c) => c,
            None => {
                let duration = start.elapsed().as_secs_f64();
                error!("Failed to get Redis connection for get_stats");
                self.metrics
                    .record_error(CacheOperation::GetStats, duration);
                return Err("Failed to get Redis connection".to_string());
            }
        };

        let info_result: redis::RedisResult<String> =
            redis::cmd("INFO").arg("stats").query_async(&mut conn).await;

        let memory_result: redis::RedisResult<String> = redis::cmd("INFO")
            .arg("memory")
            .query_async(&mut conn)
            .await;

        let mut stats = CacheStats::default();

        if let Ok(info) = info_result {
            for line in info.lines() {
                if line.starts_with("total_connections_received:") {
                    if let Some(value) = line.split(':').nth(1) {
                        stats.total_connections = value.trim().parse().unwrap_or(0);
                    }
                } else if line.starts_with("total_commands_processed:") {
                    if let Some(value) = line.split(':').nth(1) {
                        stats.total_commands = value.trim().parse().unwrap_or(0);
                    }
                } else if line.starts_with("keyspace_hits:") {
                    if let Some(value) = line.split(':').nth(1) {
                        stats.keyspace_hits = value.trim().parse().unwrap_or(0);
                    }
                } else if line.starts_with("keyspace_misses:")
                    && let Some(value) = line.split(':').nth(1)
                {
                    stats.keyspace_misses = value.trim().parse().unwrap_or(0);
                }
            }
        }

        if let Ok(memory) = memory_result {
            for line in memory.lines() {
                if line.starts_with("used_memory:") {
                    if let Some(value) = line.split(':').nth(1) {
                        stats.memory_used = value.trim().parse().unwrap_or(0);
                    }
                } else if line.starts_with("used_memory_human:")
                    && let Some(value) = line.split(':').nth(1)
                {
                    stats.memory_used_human = value.trim().to_string();
                }
            }
        }

        let dbsize_result: redis::RedisResult<u64> =
            redis::cmd("DBSIZE").query_async(&mut conn).await;

        if let Ok(dbsize) = dbsize_result {
            stats.total_keys = dbsize;
        }

        let total_requests = stats.keyspace_hits + stats.keyspace_misses;
        if total_requests > 0 {
            stats.hit_rate = (stats.keyspace_hits as f64 / total_requests as f64) * 100.0;
        }

        // Catat metrik keberhasilan setelah operasi selesai
        let duration = start.elapsed().as_secs_f64();
        self.metrics
            .record_success(CacheOperation::GetStats, duration);
        info!("Successfully fetched cache stats in {:.3}s", duration);

        Ok(stats)
    }

    pub fn get_ref_count(&self) -> usize {
        Arc::strong_count(&self.redis_pool)
    }
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub memory_used: u64,
    pub memory_used_human: String,
    pub total_connections: u64,
    pub total_commands: u64,
    pub total_keys: u64,
    pub keyspace_hits: u64,
    pub keyspace_misses: u64,
    pub hit_rate: f64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache Stats:\n\
             - Memory: {} ({})\n\
             - Keys: {}\n\
             - Connections: {}\n\
             - Commands: {}\n\
             - Hit Rate: {:.2}% ({} hits / {} misses)",
            self.memory_used_human,
            self.memory_used,
            self.total_keys,
            self.total_connections,
            self.total_commands,
            self.hit_rate,
            self.keyspace_hits,
            self.keyspace_misses
        )
    }
}
