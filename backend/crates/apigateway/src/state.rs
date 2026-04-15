use crate::{di::DependenciesInject, service::GrpcClients};
use anyhow::{Context, Result};
use shared::abstract_trait::rate_limit::DynRateLimitMiddleware;
use shared::abstract_trait::session::DynSessionMiddleware;
use shared::cache::session::SessionStore;
use shared::config::GatewayLimiterConfig;
use shared::resilience::{GatewayCircuitBreaker, GatewayRequestLimiter};
use shared::{
    abstract_trait::jwt::DynJwtService,
    config::{GrpcServiceEndpoints, JwtConfig},
    observability::run_metrics_collector,
};
use shared::{
    cache::{CacheStore, rate_limit::RateLimiter},
    config::{RedisConfig, RedisPool},
    observability::CacheMetricsCore,
};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub struct AppState {
    pub jwt_config: DynJwtService,
    pub rate_limit: DynRateLimitMiddleware,
    pub session: DynSessionMiddleware,
    pub di_container: Arc<DependenciesInject>,
    pub circuit_breaker: Arc<GatewayCircuitBreaker>,
    pub request_limiter: Arc<GatewayRequestLimiter>,
    pub cache_store: Arc<CacheStore>,
}

impl AppState {
    pub async fn new(jwt_secret: &str) -> Result<Self> {
        let jwt_config = Arc::new(JwtConfig::new(jwt_secret)) as DynJwtService;

        let grpc_config = GrpcServiceEndpoints::init().context("failed config grpc")?;

        let redis_config = RedisConfig::new();
        let redis = RedisPool::new(&redis_config).context("Failed to connect to Redis")?;

        redis.ping().await.context("Failed to ping Redis server")?;

        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));

        let rate_limiter_middleware =
            Arc::new(RateLimiter::new(redis.pool.clone())) as DynRateLimitMiddleware;
        let session_middleware =
            Arc::new(SessionStore::new(redis.pool.clone())) as DynSessionMiddleware;

        let clients = GrpcClients::init(grpc_config)
            .await
            .context("failed grpc client")?;

        let di_container = Arc::new(
            DependenciesInject::new(clients, cache_store.clone())
                .context("Failed to initialized depencency injection container")?,
        );

        let cfg = GatewayLimiterConfig::from_env();

        let circuit_breaker = Arc::new(GatewayCircuitBreaker::new(
            cfg.cb_max_failures,
            cfg.cb_reset_timeout_sec,
        ));

        let request_limiter = Arc::new(GatewayRequestLimiter::new(cfg.rate_limit));

        Self::spawn_monitoring_task(
            Arc::clone(&circuit_breaker),
            Arc::clone(&request_limiter),
            Arc::clone(&di_container),
        );

        tokio::spawn(run_metrics_collector());

        Ok(Self {
            jwt_config,
            di_container: di_container.clone(),
            session: session_middleware,
            rate_limit: rate_limiter_middleware,
            circuit_breaker,
            request_limiter,
            cache_store,
        })
    }

    fn spawn_monitoring_task(
        circuit_breaker: Arc<GatewayCircuitBreaker>,
        request_limiter: Arc<GatewayRequestLimiter>,
        di_container: Arc<DependenciesInject>,
    ) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;

                let cache_refs = di_container.cache_store.get_ref_count();
                let circuit_open = circuit_breaker.is_open();
                let failure_count = circuit_breaker.get_failure_count();
                let success_count = circuit_breaker.get_success_count();
                let available_permits = request_limiter.available_permits();
                let max_permits = request_limiter.max_concurrent();

                info!(
                    "📊 Gateway Metrics: cache_refs={}, permits={}/{}, circuit={}, failures={}, successes={}",
                    cache_refs,
                    available_permits,
                    max_permits,
                    if circuit_open {
                        "OPEN🔴"
                    } else {
                        "CLOSED🟢"
                    },
                    failure_count,
                    success_count
                );

                if circuit_open {
                    info!("🔴 Circuit breaker is OPEN - rejecting all requests");
                }

                if available_permits < 100 {
                    info!(
                        "⚠️  Low available permits: {} / {}",
                        available_permits, max_permits
                    );
                }

                if cache_refs > 500 {
                    info!("⚠️  High cache references: {}", cache_refs);
                }
            }
        });
    }
}
