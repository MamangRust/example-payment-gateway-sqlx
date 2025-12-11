use crate::{di::DependenciesInject, service::GrpcClients};
use anyhow::{Context, Result};
use shared::abstract_trait::rate_limit::DynRateLimitMiddleware;
use shared::abstract_trait::session::DynSessionMiddleware;
use shared::cache::session::SessionStore;
use shared::{
    abstract_trait::jwt::DynJwtService,
    config::{GrpcClientConfig, JwtConfig},
    utils::{SystemMetrics, run_metrics_collector},
};
use shared::{
    cache::rate_limit::RateLimiter,
    config::{RedisConfig, RedisPool},
};
use std::sync::Arc;

pub struct AppState {
    pub jwt_config: DynJwtService,
    pub rate_limit: DynRateLimitMiddleware,
    pub di_container: DependenciesInject,
    pub system_metrics: Arc<SystemMetrics>,
    pub session: DynSessionMiddleware,
}

impl AppState {
    pub async fn new(jwt_secret: &str) -> Result<Self> {
        let jwt_config = Arc::new(JwtConfig::new(jwt_secret)) as DynJwtService;
        let system_metrics = Arc::new(SystemMetrics::new());
        let grpc_config = GrpcClientConfig::init().context("failed config grpc")?;

        let redis_config = RedisConfig::new("redis".into(), 6379, 0, Some("dragon_knight".into()));
        let redis = RedisPool::new(&redis_config).context("Failed to connect to Redis")?;

        redis.ping().await.context("Failed to ping Redis server")?;

        let rate_limiter_middleware =
            Arc::new(RateLimiter::new(redis.pool.clone())) as DynRateLimitMiddleware;
        let session_middleware =
            Arc::new(SessionStore::new(redis.pool.clone())) as DynSessionMiddleware;

        let clients = GrpcClients::init(grpc_config)
            .await
            .context("failed grpc client")?;

        let di_container = DependenciesInject::new(clients, redis)
            .context("Failed to initialized depencency injection container")?;

        tokio::spawn(run_metrics_collector(system_metrics.clone()));

        Ok(Self {
            jwt_config,
            di_container,
            session: session_middleware,
            rate_limit: rate_limiter_middleware,
            system_metrics,
        })
    }
}
