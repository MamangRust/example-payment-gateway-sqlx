use anyhow::{Context, Result};
use deadpool_redis::{
    Config as DeadpoolRedisConfig, Connection, Pool, PoolError, Runtime, redis::cmd,
};
use tracing::info;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: Option<String>,
}

impl RedisConfig {
    pub fn new(host: String, port: u16, db: u8, password: Option<String>) -> Self {
        Self {
            host,
            port,
            db,
            password,
        }
    }
    pub fn url(&self) -> String {
        match &self.password {
            Some(pw) => format!("redis://:{}@{}:{}/{}", pw, self.host, self.port, self.db),
            None => format!("redis://{}:{}/{}", self.host, self.port, self.db),
        }
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
