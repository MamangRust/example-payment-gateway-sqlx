use anyhow::{Context, Result};
use shared::config::RedisPool;
use shared::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        transfer::{
            repository::{
                command::DynTransferCommandRepository,
                query::DynTransferQueryRepository,
                stats::{
                    amount::DynTransferStatsAmountRepository,
                    status::DynTransferStatsStatusRepository,
                },
                statsbycard::{
                    amount::DynTransferStatsAmountByCardRepository,
                    status::DynTransferStatsStatusByCardRepository,
                },
            },
            service::{
                command::DynTransferCommandService,
                query::DynTransferQueryService,
                stats::{
                    amount::DynTransferStatsAmountService, status::DynTransferStatsStatusService,
                },
                statsbycard::{
                    amount::DynTransferStatsAmountByCardService,
                    status::DynTransferStatsStatusByCardService,
                },
            },
        },
    },
    cache::CacheStore,
    config::ConnectionPool,
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        card::query::CardQueryRepository,
        saldo::{command::SaldoCommandRepository, query::SaldoQueryRepository},
        transfer::{
            command::TransferCommandRepository,
            query::TransferQueryRepository,
            stats::{amount::TransferStatsAmountRepository, status::TransferStatsStatusRepository},
            statsbycard::{
                amount::TransferStatsAmountByCardRepository,
                status::TransferStatsStatusByCardRepository,
            },
        },
    },
    service::transfer::{
        command::{TransferCommandService, TransferCommandServiceDeps},
        query::TransferQueryService,
        stats::{amount::TransferStatsAmountService, status::TransferStatsStatusService},
        statsbycard::{
            amount::TransferStatsAmountByCardService, status::TransferStatsStatusByCardService,
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
    pub transfer_command: DynTransferCommandService,
    pub transfer_query: DynTransferQueryService,
    pub transfer_stats_amount: DynTransferStatsAmountService,
    pub transfer_stats_status: DynTransferStatsStatusService,
    pub transfer_stats_amount_by_card: DynTransferStatsAmountByCardService,
    pub transfer_stats_status_by_card: DynTransferStatsStatusByCardService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DependenciesInject");

        debug_struct
            .field("transfer_command", &"DynTransferCommandService")
            .field("transfer_query", &"DynTransferQueryService")
            .field("transfer_stats_amount", &"DynTransferStatsAmountService")
            .field("transfer_stats_status", &"DynTransferStatsStatusService")
            .field(
                "transfer_stats_amount_by_card",
                &"DynTransferStatsAmountByCardService",
            )
            .field(
                "transfer_stats_status_by_card",
                &"DynTransferStatsStatusByCardService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));

        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));
        let tracing_metrics = Arc::new(
            TracingMetricsCore::new("transfer-service").context("failed initialize tracing")?,
        );

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let transfer_query_repo =
            Arc::new(TransferQueryRepository::new(db.clone())) as DynTransferQueryRepository;

        let transfer_query = Arc::new(
            TransferQueryService::new(transfer_query_repo.clone(), &shared)
                .context("failed to initialize transfer query service")?,
        ) as DynTransferQueryService;

        let transfer_command_repo =
            Arc::new(TransferCommandRepository::new(db.clone())) as DynTransferCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let command_deps = TransferCommandServiceDeps {
            card_query: card_query_repo,
            saldo_query: saldo_query_repo,
            saldo_command: saldo_command_repo,
            query: transfer_query_repo.clone(),
            command: transfer_command_repo.clone(),
        };
        let transfer_command = Arc::new(
            TransferCommandService::new(command_deps, &shared)
                .context("failed to initialize transfer command service")?,
        ) as DynTransferCommandService;

        let amount_repo = Arc::new(TransferStatsAmountRepository::new(db.clone()))
            as DynTransferStatsAmountRepository;
        let transfer_stats_amount = Arc::new(
            TransferStatsAmountService::new(amount_repo.clone(), &shared)
                .context("failed to initialize transfer stats amount service")?,
        ) as DynTransferStatsAmountService;

        let status_repo = Arc::new(TransferStatsStatusRepository::new(db.clone()))
            as DynTransferStatsStatusRepository;
        let transfer_stats_status = Arc::new(
            TransferStatsStatusService::new(status_repo.clone(), &shared)
                .context("failed to initialize transfer stats status service")?,
        ) as DynTransferStatsStatusService;

        let amount_by_card_repo = Arc::new(TransferStatsAmountByCardRepository::new(db.clone()))
            as DynTransferStatsAmountByCardRepository;
        let transfer_stats_amount_by_card = Arc::new(
            TransferStatsAmountByCardService::new(amount_by_card_repo.clone(), &shared)
                .context("failed to initialize transfer stats amount by card service")?,
        ) as DynTransferStatsAmountByCardService;

        let status_by_card_repo = Arc::new(TransferStatsStatusByCardRepository::new(db.clone()))
            as DynTransferStatsStatusByCardRepository;
        let transfer_stats_status_by_card = Arc::new(
            TransferStatsStatusByCardService::new(status_by_card_repo.clone(), &shared)
                .context("failed to initialize transfer stats status by card service")?,
        ) as DynTransferStatsStatusByCardService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            transfer_command,
            transfer_query,
            transfer_stats_amount,
            transfer_stats_status,
            transfer_stats_amount_by_card,
            transfer_stats_status_by_card,
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
