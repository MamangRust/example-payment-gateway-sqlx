use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        topup::{
            repository::{
                command::DynTopupCommandRepository,
                query::DynTopupQueryRepository,
                stats::{
                    amount::DynTopupStatsAmountRepository, method::DynTopupStatsMethodRepository,
                    status::DynTopupStatsStatusRepository,
                },
                statsbycard::{
                    amount::DynTopupStatsAmountByCardRepository,
                    method::DynTopupStatsMethodByCardRepository,
                    status::DynTopupStatsStatusByCardRepository,
                },
            },
            service::{
                command::DynTopupCommandService,
                query::DynTopupQueryService,
                stats::{
                    amount::DynTopupStatsAmountService, method::DynTopupStatsMethodService,
                    status::DynTopupStatsStatusService,
                },
                statsbycard::{
                    amount::DynTopupStatsAmountByCardService,
                    method::DynTopupStatsMethodByCardService,
                    status::DynTopupStatsStatusByCardService,
                },
            },
        },
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        card::query::CardQueryRepository,
        saldo::{command::SaldoCommandRepository, query::SaldoQueryRepository},
        topup::{
            command::TopupCommandRepository,
            query::TopupQueryRepository,
            stats::{
                amount::TopupStatsAmountRepository, method::TopupStatsMethodRepository,
                status::TopupStatsStatusRepository,
            },
            statsbycard::{
                amount::TopupStatsAmountByCardRepository, method::TopupStatsMethodByCardRepository,
                status::TopupStatsStatusByCardRepository,
            },
        },
    },
    service::topup::{
        command::{TopupCommandService, TopupCommandServiceDeps},
        query::TopupQueryService,
        stats::{
            amount::TopupStatsAmountService, method::TopupStatsMethodService,
            status::TopupStatsStatusService,
        },
        statsbycard::{
            amount::TopupStatsAmountByCardService, method::TopupStatsMethodByCardService,
            status::TopupStatsStatusByCardService,
        },
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
    pub topup_command: DynTopupCommandService,
    pub topup_query: DynTopupQueryService,
    pub topup_stats_amount: DynTopupStatsAmountService,
    pub topup_stats_method: DynTopupStatsMethodService,
    pub topup_stats_status: DynTopupStatsStatusService,
    pub topup_stats_amount_by_card: DynTopupStatsAmountByCardService,
    pub topup_stats_method_by_card: DynTopupStatsMethodByCardService,
    pub topup_stats_status_by_card: DynTopupStatsStatusByCardService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DependenciesInject");

        debug_struct
            .field("topup_command", &"DynTopupCommandService")
            .field("topup_query", &"DynTopupQueryService")
            .field("topup_stats_amount", &"DynTopupStatsAmountService")
            .field("topup_stats_method", &"DynTopupStatsMethodService")
            .field("topup_stats_status", &"DynTopupStatsStatusService")
            .field(
                "topup_stats_amount_by_card",
                &"DynTopupStatsAmountByCardService",
            )
            .field(
                "topup_stats_method_by_card",
                &"DynTopupStatsMethodByCardService",
            )
            .field(
                "topup_stats_status_by_card",
                &"DynTopupStatsStatusByCardService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));
        let tracing_metrics = Arc::new(
            TracingMetricsCore::new("topup-service").context("failed initialize tracing")?,
        );

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let topup_query_repo =
            Arc::new(TopupQueryRepository::new(db.clone())) as DynTopupQueryRepository;
        let topup_query = Arc::new(
            TopupQueryService::new(topup_query_repo.clone(), &shared)
                .context("failed to initialize topup query service")?,
        ) as DynTopupQueryService;

        let topup_command_repo =
            Arc::new(TopupCommandRepository::new(db.clone())) as DynTopupCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let command_deps = TopupCommandServiceDeps {
            card_query: card_query_repo,
            saldo_query: saldo_query_repo,
            saldo_command: saldo_command_repo,
            query: topup_query_repo.clone(),
            command: topup_command_repo.clone(),
        };
        let topup_command = Arc::new(
            TopupCommandService::new(command_deps, &shared)
                .context("failed to initialize topup command service")?,
        ) as DynTopupCommandService;

        let amount_repo =
            Arc::new(TopupStatsAmountRepository::new(db.clone())) as DynTopupStatsAmountRepository;
        let topup_stats_amount = Arc::new(
            TopupStatsAmountService::new(amount_repo.clone(), &shared)
                .context("failed to initialize topup stats amount service")?,
        ) as DynTopupStatsAmountService;

        let method_repo =
            Arc::new(TopupStatsMethodRepository::new(db.clone())) as DynTopupStatsMethodRepository;
        let topup_stats_method = Arc::new(
            TopupStatsMethodService::new(method_repo.clone(), &shared)
                .context("failed to initialize topup stats method service")?,
        ) as DynTopupStatsMethodService;

        let status_repo =
            Arc::new(TopupStatsStatusRepository::new(db.clone())) as DynTopupStatsStatusRepository;
        let topup_stats_status = Arc::new(
            TopupStatsStatusService::new(status_repo.clone(), &shared)
                .context("failed to initialize topup stats status service")?,
        ) as DynTopupStatsStatusService;

        let amount_by_card_repo = Arc::new(TopupStatsAmountByCardRepository::new(db.clone()))
            as DynTopupStatsAmountByCardRepository;
        let topup_stats_amount_by_card = Arc::new(
            TopupStatsAmountByCardService::new(amount_by_card_repo.clone(), &shared)
                .context("failed to initialize topup stats amount by card service")?,
        ) as DynTopupStatsAmountByCardService;

        let method_by_card_repo = Arc::new(TopupStatsMethodByCardRepository::new(db.clone()))
            as DynTopupStatsMethodByCardRepository;
        let topup_stats_method_by_card = Arc::new(
            TopupStatsMethodByCardService::new(method_by_card_repo.clone(), &shared)
                .context("failed to initialize topup stats method by card service")?,
        ) as DynTopupStatsMethodByCardService;

        let status_by_card_repo = Arc::new(TopupStatsStatusByCardRepository::new(db.clone()))
            as DynTopupStatsStatusByCardRepository;
        let topup_stats_status_by_card = Arc::new(
            TopupStatsStatusByCardService::new(status_by_card_repo.clone(), &shared)
                .context("failed to initialize topup stats status by card service")?,
        ) as DynTopupStatsStatusByCardService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            topup_command,
            topup_query,
            topup_stats_amount,
            topup_stats_method,
            topup_stats_status,
            topup_stats_amount_by_card,
            topup_stats_method_by_card,
            topup_stats_status_by_card,
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
