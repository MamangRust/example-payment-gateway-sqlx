use crate::di::{DependenciesInject, DependenciesInjectDeps};
use anyhow::{Context, Result};
use shared::{
    abstract_trait::{hashing::DynHashing, jwt::DynJwtService},
    config::{Config, ConnectionPool, Hashing, JwtConfig, RedisConfig, RedisPool},
    utils::{SystemMetrics, run_metrics_collector},
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
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
    pub async fn new(pool: ConnectionPool, config: Config) -> Result<Self> {
        let jwt_config = Arc::new(JwtConfig::new(&config.jwt_secret)) as DynJwtService;
        let hashing = Arc::new(Hashing::new()) as DynHashing;
        let system_metrics = Arc::new(SystemMetrics::new());

        let config = RedisConfig::new("redis".into(), 6379, 1, Some("dragon_knight".into()));

        let redis = RedisPool::new(&config).context("Failed to connect to Redis")?;

        redis.ping().await.context("Failed to ping Redis server")?;

        let deps = DependenciesInjectDeps {
            pool: pool.clone(),
            hash: hashing,
            jwt_config,
            redis: redis.clone(),
        };

        let di_container = DependenciesInject::new(deps)
            .context("Failed to initialize dependency injection container")?;

        tokio::spawn(run_metrics_collector(system_metrics.clone()));

        Ok(Self {
            di_container,
            system_metrics,
        })
    }
}
