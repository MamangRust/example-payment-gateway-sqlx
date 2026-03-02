use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        withdraw::{
            repository::{
                command::DynWithdrawCommandRepository,
                query::DynWithdrawQueryRepository,
                stats::{
                    amount::DynWithdrawStatsAmountRepository,
                    status::DynWithdrawStatsStatusRepository,
                },
                statsbycard::{
                    amount::DynWithdrawStatsAmountByCardRepository,
                    status::DynWithdrawStatsStatusByCardRepository,
                },
            },
            service::{
                command::DynWithdrawCommandService,
                query::DynWithdrawQueryService,
                stats::{
                    amount::DynWithdrawStatsAmountService, status::DynWithdrawStatsStatusService,
                },
                statsbycard::{
                    amount::DynWithdrawStatsAmountByCardService,
                    status::DynWithdrawStatsStatusByCardService,
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
        withdraw::{
            command::WithdrawCommandRepository,
            query::WithdrawQueryRepository,
            stats::{amount::WithdrawStatsAmountRepository, status::WithdrawStatsStatusRepository},
            statsbycard::{
                amount::WithdrawStatsAmountByCardRepository,
                status::WithdrawStatsStatusByCardRepository,
            },
        },
    },
    service::withdraw::{
        command::{WithdrawCommandService, WithdrawCommandServiceDeps},
        query::WithdrawQueryService,
        stats::{amount::WithdrawStatsAmountService, status::WithdrawStatsStatusService},
        statsbycard::{
            amount::WithdrawStatsAmountByCardService, status::WithdrawStatsStatusByCardService,
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
    pub withdraw_command: DynWithdrawCommandService,
    pub withdraw_query: DynWithdrawQueryService,
    pub withdraw_stats_amount: DynWithdrawStatsAmountService,
    pub withdraw_stats_status: DynWithdrawStatsStatusService,
    pub withdraw_stats_amount_by_card: DynWithdrawStatsAmountByCardService,
    pub withdraw_stats_status_by_card: DynWithdrawStatsStatusByCardService,
    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DependenciesInject");

        debug_struct
            .field("withdraw_command", &"DynWithdrawCommandService")
            .field("withdraw_query", &"DynWithdrawQueryService")
            .field("withdraw_stats_amount", &"DynWithdrawStatsAmountService")
            .field("withdraw_stats_status", &"DynWithdrawStatsStatusService")
            .field(
                "withdraw_stats_amount_by_card",
                &"DynWithdrawStatsAmountByCardService",
            )
            .field(
                "withdraw_stats_status_by_card",
                &"DynWithdrawStatsStatusByCardService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));

        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));

        let tracing_metrics = Arc::new(
            TracingMetricsCore::new("withdraw-service").context("failed initialize tracing")?,
        );

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let withdraw_query_repo =
            Arc::new(WithdrawQueryRepository::new(db.clone())) as DynWithdrawQueryRepository;

        let withdraw_query = Arc::new(
            WithdrawQueryService::new(withdraw_query_repo.clone(), &shared)
                .context("failed to initialize withdraw query service")?,
        ) as DynWithdrawQueryService;

        let withdraw_command_repo =
            Arc::new(WithdrawCommandRepository::new(db.clone())) as DynWithdrawCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let command_deps = WithdrawCommandServiceDeps {
            query: withdraw_query_repo.clone(),
            command: withdraw_command_repo.clone(),
            card_query: card_query_repo,
            saldo_query: saldo_query_repo,
            saldo_command: saldo_command_repo,
        };
        let withdraw_command = Arc::new(
            WithdrawCommandService::new(command_deps, &shared)
                .context("failed to initialize withdraw command service")?,
        ) as DynWithdrawCommandService;

        let amount_repo = Arc::new(WithdrawStatsAmountRepository::new(db.clone()))
            as DynWithdrawStatsAmountRepository;
        let withdraw_stats_amount = Arc::new(
            WithdrawStatsAmountService::new(amount_repo.clone(), &shared)
                .context("failed to initialize withdraw stats amount service")?,
        ) as DynWithdrawStatsAmountService;

        let status_repo = Arc::new(WithdrawStatsStatusRepository::new(db.clone()))
            as DynWithdrawStatsStatusRepository;
        let withdraw_stats_status = Arc::new(
            WithdrawStatsStatusService::new(status_repo.clone(), &shared)
                .context("failed to initialize withdraw stats status service")?,
        ) as DynWithdrawStatsStatusService;

        let amount_by_card_repo = Arc::new(WithdrawStatsAmountByCardRepository::new(db.clone()))
            as DynWithdrawStatsAmountByCardRepository;
        let withdraw_stats_amount_by_card = Arc::new(
            WithdrawStatsAmountByCardService::new(amount_by_card_repo.clone(), &shared)
                .context("failed to initialize withdraw stats amount by card service")?,
        ) as DynWithdrawStatsAmountByCardService;

        let status_by_card_repo = Arc::new(WithdrawStatsStatusByCardRepository::new(db.clone()))
            as DynWithdrawStatsStatusByCardRepository;
        let withdraw_stats_status_by_card = Arc::new(
            WithdrawStatsStatusByCardService::new(status_by_card_repo.clone(), &shared)
                .context("failed to initialize withdraw stats status by card service")?,
        ) as DynWithdrawStatsStatusByCardService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            withdraw_command,
            withdraw_query,
            withdraw_stats_amount,
            withdraw_stats_status,
            withdraw_stats_amount_by_card,
            withdraw_stats_status_by_card,
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
