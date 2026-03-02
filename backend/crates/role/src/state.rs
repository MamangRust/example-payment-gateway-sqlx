use crate::di::DependenciesInject;
use anyhow::{Context, Result};
use shared::{
    config::{ConnectionPool, RedisConfig, RedisPool},
    observability::run_metrics_collector,
    resilience::{CircuitBreaker, LoadMonitor},
};
use std::{fmt, sync::Arc, time::Duration};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AppState {
    pub di_container: Arc<DependenciesInject>,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub load_monitor: Arc<LoadMonitor>,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("deps", &self.di_container)
            .finish()
    }
}

impl AppState {
    pub async fn new(pool: ConnectionPool) -> Result<Self> {
        let redis_config = RedisConfig::new();

        let redis = RedisPool::new(&redis_config).context("Failed to connect to Redis")?;

        redis.ping().await.context("Failed to ping Redis server")?;

        let di_container = Arc::new(
            DependenciesInject::new(pool.clone(), redis.clone())
                .context("Failed to initialize dependency injection container")?,
        );

        let circuit_breaker = Arc::new(CircuitBreaker::new());
        let load_monitor = Arc::new(LoadMonitor::new());

        Self::spawn_load_monitoring(
            load_monitor.clone(),
            di_container.clone(),
            circuit_breaker.clone(),
        );

        tokio::spawn(run_metrics_collector());


        Ok(Self {
            di_container,
            circuit_breaker,
            load_monitor,
        })
    }

    fn spawn_load_monitoring(
        monitor: Arc<LoadMonitor>,
        di: Arc<DependenciesInject>,
        cb: Arc<CircuitBreaker>,
    ) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            let mut high_load_duration = 0u64;
            let mut normal_load_duration = 0u64;

            loop {
                interval.tick().await;

                let rps = monitor.get_current_rps().await;
                let metrics = di.get_metrics();

                info!(
                    "📈 Metrics - RPS: {}, Available Permits: {}, Cache Refs: {}",
                    rps, metrics.available_permits, metrics.cache_ref_count
                );

                if rps > 4000 {
                    high_load_duration += 10;
                    normal_load_duration = 0;

                    warn!("⚠️  High RPS detected: {} req/s", rps);

                    if high_load_duration >= 60 {
                        warn!(
                            "🔥 Sustained high load for {} seconds, enabling circuit breaker",
                            high_load_duration
                        );
                    }
                } else if rps < 1000 {
                    if high_load_duration > 0 {
                        high_load_duration = high_load_duration.saturating_sub(10);
                    }
                    normal_load_duration += 10;

                    if normal_load_duration >= 120 && metrics.cache_ref_count > 100 {
                        info!("🧹 Triggering aggressive cleanup after load spike");
                        normal_load_duration = 0;

                        if let Err(e) = di.trigger_cleanup().await {
                            error!("Failed to trigger cleanup: {:?}", e);
                        }

                        if let Err(e) = di.invalidate_cache_pattern("role:*").await {
                            error!("Failed to invalidate cache: {:?}", e);
                        }
                    }
                } else {
                    if high_load_duration > 0 {
                        high_load_duration = high_load_duration.saturating_sub(5);
                    }
                    normal_load_duration = normal_load_duration.saturating_sub(5);
                }

                if rps < 2000 && cb.is_open() {
                    info!("✅ Load normalized, resetting circuit breaker");
                    cb.reset();
                }
            }
        });
    }
}
