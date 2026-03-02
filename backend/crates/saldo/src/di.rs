use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::{
            repository::{
                command::DynSaldoCommandRepository,
                query::DynSaldoQueryRepository,
                stats::{
                    balance::DynSaldoBalanceRepository, total::DynSaldoTotalBalanceRepository,
                },
            },
            service::{
                command::DynSaldoCommandService,
                query::DynSaldoQueryService,
                stats::{balance::DynSaldoBalanceService, total::DynSaldoTotalBalanceService},
            },
        },
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        card::query::CardQueryRepository,
        saldo::{
            command::SaldoCommandRepository,
            query::SaldoQueryRepository,
            stats::{balance::SaldoBalanceRepository, total::SaldoTotalBalanceRepository},
        },
    },
    service::saldo::{
        command::{SaldoCommandService, SaldoCommandServiceDeps},
        query::SaldoQueryService,
        stats::{balance::SaldoBalanceService, total::SaldoTotalBalanceService},
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
    pub saldo_command: DynSaldoCommandService,
    pub saldo_query: DynSaldoQueryService,
    pub saldo_balance: DynSaldoBalanceService,
    pub saldo_total_balance: DynSaldoTotalBalanceService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("saldo_command", &"DynSaldoCommandService")
            .field("saldo_query", &"DynSaldoQueryService")
            .field("saldo_balance", &"DynSaldoBalanceService")
            .field("saldo_total_balance", &"DynSaldoTotalBalanceService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));

        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));
        let tracing_metrics = Arc::new(
            TracingMetricsCore::new("saldo-service").context("failed initialize tracing")?,
        );

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_query = Arc::new(
            SaldoQueryService::new(saldo_query_repo.clone(), &shared)
                .context("failed to initialize saldo query service")?,
        ) as DynSaldoQueryService;

        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let command_service_deps = SaldoCommandServiceDeps {
            card_query: card_query_repo,
            command: saldo_command_repo,
        };
        let saldo_command = Arc::new(
            SaldoCommandService::new(command_service_deps, &shared)
                .context("failed to initialize saldo command service")?,
        ) as DynSaldoCommandService;

        let balance_repo =
            Arc::new(SaldoBalanceRepository::new(db.clone())) as DynSaldoBalanceRepository;
        let saldo_balance = Arc::new(
            SaldoBalanceService::new(balance_repo.clone(), &shared)
                .context("failed to initialize saldo balance service")?,
        ) as DynSaldoBalanceService;

        let total_repo = Arc::new(SaldoTotalBalanceRepository::new(db.clone()))
            as DynSaldoTotalBalanceRepository;
        let saldo_total_balance = Arc::new(
            SaldoTotalBalanceService::new(total_repo.clone(), &shared)
                .context("failed to initialize saldo total balance service")?,
        ) as DynSaldoTotalBalanceService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            saldo_command,
            saldo_query,
            saldo_balance,
            saldo_total_balance,
            cache_store: shared.cache_store,
            request_limiter: Arc::new(Semaphore::new(1000)),
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
