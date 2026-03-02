use anyhow::{Context, Result};
use shared::{
    abstract_trait::role::{
        repository::{command::DynRoleCommandRepository, query::DynRoleQueryRepository},
        service::{command::DynRoleCommandService, query::DynRoleQueryService},
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::role::{command::RoleCommandRepository, query::RoleQueryRepository},
    service::role::{command::RoleCommandService, query::RoleQueryService},
};
use std::{fmt, sync::Arc, time::Duration};
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
pub struct DependencyMetrics {
    pub available_permits: usize,
    pub cache_ref_count: usize,
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub role_query: DynRoleQueryService,
    pub role_command: DynRoleCommandService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("role_query", &"DynRoleQueryService")
            .field("role_command", &"DynRoleCommandService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));

        let tracing_metrics =
            Arc::new(TracingMetricsCore::new("role-service").context("failed initialize tracing")?);

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let role_query_repo =
            Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;

        let role_query = Arc::new(
            RoleQueryService::new(role_query_repo, &shared)
                .context("failed to initialize role query service")?,
        ) as DynRoleQueryService;

        let role_command_repo =
            Arc::new(RoleCommandRepository::new(db.clone())) as DynRoleCommandRepository;
        let role_command = Arc::new(
            RoleCommandService::new(role_command_repo, &shared)
                .context("failed to initialize role command service")?,
        ) as DynRoleCommandService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            role_query,
            role_command,
            request_limiter: Arc::new(Semaphore::new(1000)),
            cache_store: shared.cache_store,
        })
    }

    fn spawn_monitoring_task(cache: Arc<CacheStore>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let ref_count = Arc::strong_count(&cache);
                if ref_count > 1000 {
                    warn!(
                        "⚠️  High reference count detected on CacheStore: {}",
                        ref_count
                    );
                } else {
                    info!("📊 CacheStore reference count: {}", ref_count);
                }
            }
        });
    }

    fn spawn_cleanup_task(cache: Arc<CacheStore>) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(120));
            loop {
                interval.tick().await;
                info!("🧹 Running periodic cache cleanup...");

                let _ = cache.clear_expired().await;

                let ref_count = Arc::strong_count(&cache);
                info!("✅ Cleanup completed. Current ref count: {}", ref_count);
            }
        });
    }

    pub async fn trigger_cleanup(&self) -> Result<()> {
        info!("🧹 Triggering manual cleanup...");

        match self.cache_store.clear_expired().await {
            Ok(scanned) => info!("✅ Manual cleanup scanned {} keys", scanned),
            Err(e) => error!("❌ Manual cleanup failed: {}", e),
        }

        if let Ok(stats) = self.cache_store.get_stats().await {
            info!("📊 Post-cleanup stats:\n{}", stats);
        }

        Ok(())
    }

    pub async fn invalidate_cache_pattern(&self, pattern: &str) -> Result<usize> {
        self.cache_store
            .invalidate_pattern(pattern)
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to invalidate cache pattern")
    }

    pub fn get_metrics(&self) -> DependencyMetrics {
        DependencyMetrics {
            available_permits: self.request_limiter.available_permits(),
            cache_ref_count: Arc::strong_count(&self.cache_store),
        }
    }
}
