use anyhow::{Context, Result};
use deadpool_redis::{
    Config as DeadpoolRedisConfig, Connection, Pool, PoolError, Runtime, redis::cmd,
};
use std::env;
use tracing::info;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl RedisConfig {
    pub fn new() -> Self {
        let host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("REDIS_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(6379);

        let db = env::var("REDIS_DB")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(0);

        let user = env::var("REDIS_USER").ok().filter(|v| !v.is_empty());

        let password = env::var("REDIS_PASSWORD").ok().filter(|v| !v.is_empty());

        Self {
            host,
            port,
            db,
            user,
            password,
        }
    }

    pub fn url(&self) -> String {
        match (&self.user, &self.password) {
            (Some(user), Some(pw)) => {
                format!(
                    "redis://{}:{}@{}:{}/{}",
                    user, pw, self.host, self.port, self.db
                )
            }
            (None, Some(pw)) => {
                format!("redis://:{}@{}:{}/{}", pw, self.host, self.port, self.db)
            }
            _ => {
                format!("redis://{}:{}/{}", self.host, self.port, self.db)
            }
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig::new()
    }
}

#[derive(Clone)]
pub struct RedisPool {
    pub pool: Pool,
}

impl RedisPool {
    pub fn new(config: &RedisConfig) -> Result<Self> {
        info!("Creating redis pool (deadpool-redis)");

        let pool_cfg = DeadpoolRedisConfig::from_url(config.url());

        let pool = pool_cfg
            .create_pool(Some(Runtime::Tokio1))
            .context("failed create redis connection pool")?;

        Ok(Self { pool })
    }

    pub async fn get_conn(&self) -> Result<Connection, PoolError> {
        self.pool.get().await
    }

    pub async fn ping(&self) -> Result<(), PoolError> {
        let mut conn = self.get_conn().await?;
        info!("Pinging redis (deadpool-redis)");
        cmd("PING").query_async::<()>(&mut conn).await?;
        info!("Pinged redis (deadpool-redis)");
        Ok(())
    }
}
