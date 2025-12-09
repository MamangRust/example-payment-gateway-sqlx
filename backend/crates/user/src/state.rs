use crate::di::DependenciesInject;
use anyhow::{Context, Result};
use shared::{
    config::{ConnectionPool, RedisConfig, RedisPool},
    utils::{SystemMetrics, run_metrics_collector},
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub di_container: Arc<DependenciesInject>,
    pub system_metrics: Arc<SystemMetrics>,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("deps", &self.di_container)
            .field("system_metrics", &self.system_metrics)
            .finish()
    }
}

impl AppState {
    pub async fn new(pool: ConnectionPool) -> Result<Self> {
        let system_metrics = Arc::new(SystemMetrics::new());

        let config = RedisConfig::new("redis".into(), 6379, 9, Some("dragon_knight".into()));

        let redis = RedisPool::new(&config).context("Failed to connect to Redis")?;

        redis.ping().await.context("Failed to ping Redis server")?;

        let di_container = DependenciesInject::new(pool.clone(), redis)
            .context("Failed to initialize dependency injection container")?;

        tokio::spawn(run_metrics_collector(system_metrics.clone()));

        Ok(Self {
            di_container: Arc::new(di_container),
            system_metrics,
        })
    }
}
