use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        hashing::DynHashing,
        role::repository::query::DynRoleQueryRepository,
        user::{
            repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
            service::{command::DynUserCommandService, query::DynUserQueryService},
        },
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, Hashing, RedisPool},
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        role::query::RoleQueryRepository,
        user::{command::UserCommandRepository, query::UserQueryRepository},
        user_role::UserRoleRepository,
    },
    service::user::{
        command::{UserCommandService, UserCommandServiceDeps},
        query::UserQueryService,
    },
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
    pub user_command: DynUserCommandService,
    pub user_query: DynUserQueryService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("user_command_service", &"DynUserCommandService")
            .field("user_query_service", &"DynUserQueryService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));

        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));
        let hashing = Arc::new(Hashing::new()) as DynHashing;
        let tracing_metrics =
            Arc::new(TracingMetricsCore::new("user-service").context("failed initialize tracing")?);

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let user_query_repo =
            Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;
        let role_query_repo =
            Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;
        let user_role_repo =
            Arc::new(UserRoleRepository::new(db.clone())) as DynUserRoleCommandRepository;
        let user_command_repo =
            Arc::new(UserCommandRepository::new(db.clone())) as DynUserCommandRepository;

        let user_command_service_deps = UserCommandServiceDeps {
            query: user_query_repo.clone(),
            command: user_command_repo.clone(),
            hashing: hashing.clone(),
            user_role: user_role_repo.clone(),
            role: role_query_repo.clone(),
        };
        let user_command = Arc::new(
            UserCommandService::new(user_command_service_deps, &shared)
                .context("failed to initialize user command service")?,
        ) as DynUserCommandService;

        let user_query = Arc::new(
            UserQueryService::new(user_query_repo.clone(), &shared)
                .context("failed to initialize user query service")?,
        ) as DynUserQueryService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            user_command,
            user_query,
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
