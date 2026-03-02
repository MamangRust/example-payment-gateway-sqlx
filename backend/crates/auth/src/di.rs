use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        auth::service::DynAuthService,
        hashing::DynHashing,
        jwt::DynJwtService,
        refresh_token::command::DynRefreshTokenCommandRepository,
        role::repository::query::DynRoleQueryRepository,
        token::DynTokenService,
        user::repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool, ServiceLimiterConfig},
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        refresh_token::RefreshTokenCommandRepository,
        role::query::RoleQueryRepository,
        user::{command::UserCommandRepository, query::UserQueryRepository},
        user_role::UserRoleRepository,
    },
    service::{
        auth::{AuthService, AuthServiceDeps},
        token::TokenService,
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
    pub auth_service: DynAuthService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("auth_service", &"<Arc<AuthService>>")
            .finish()
    }
}

#[derive(Clone)]
pub struct DependenciesInjectDeps {
    pub pool: ConnectionPool,
    pub hash: DynHashing,
    pub jwt_config: DynJwtService,
    pub redis: RedisPool,
}

impl DependenciesInject {
    pub fn new(deps: DependenciesInjectDeps) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));
        let DependenciesInjectDeps {
            pool,
            hash,
            jwt_config,
            redis,
        } = deps;

        let user_role =
            Arc::new(UserRoleRepository::new(pool.clone())) as DynUserRoleCommandRepository;

        let user_query = Arc::new(UserQueryRepository::new(pool.clone())) as DynUserQueryRepository;
        let user_command =
            Arc::new(UserCommandRepository::new(pool.clone())) as DynUserCommandRepository;

        let role = Arc::new(RoleQueryRepository::new(pool.clone())) as DynRoleQueryRepository;

        let refresh_command = Arc::new(RefreshTokenCommandRepository::new(pool.clone()))
            as DynRefreshTokenCommandRepository;

        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));

        let tracing_metrics =
            Arc::new(TracingMetricsCore::new("auth-service").context("failed initialize tracing")?);

        let token_service = Arc::new(TokenService::new(
            jwt_config.clone(),
            refresh_command.clone(),
        )) as DynTokenService;

        let deps = AuthServiceDeps {
            tracing_metrics_core: tracing_metrics,
            query: user_query,
            command: user_command,
            jwt_config: jwt_config.clone(),
            hashing: hash,
            role,
            user_role,
            token: token_service.clone(),
            refresh_command,
            cache_store: cache_store.clone(),
        };

        let auth_service =
            Arc::new(AuthService::new(deps).context("failed initialize auth service")?)
                as DynAuthService;

        let cfg_limiter = ServiceLimiterConfig::from_env();

        Self::spawn_monitoring_task(cache_store.clone());
        Self::spawn_cleanup_task(cache_store.clone());

        Ok(Self {
            auth_service,
            request_limiter: Arc::new(Semaphore::new(cfg_limiter.max_concurrent)),
            cache_store,
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
