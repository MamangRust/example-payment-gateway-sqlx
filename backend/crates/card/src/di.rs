use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        card::{
            repository::{
                command::DynCardCommandRepository,
                dashboard::{
                    balance::DynCardDashboardBalanceRepository,
                    topup::DynCardDashboardTopupRepository,
                    transaction::DynCardDashboardTransactionRepository,
                    transfer::DynCardDashboardTransferRepository,
                    withdraw::DynCardDashboardWithdrawRepository,
                },
                query::DynCardQueryRepository,
                stats::{
                    balance::DynCardStatsBalanceRepository, topup::DynCardStatsTopupRepository,
                    transaction::DynCardStatsTransactionRepository,
                    transfer::DynCardStatsTransferRepository,
                    withdraw::DynCardStatsWithdrawRepository,
                },
                statsbycard::{
                    balance::DynCardStatsBalanceByCardRepository,
                    topup::DynCardStatsTopupByCardRepository,
                    transaction::DynCardStatsTransactionByCardRepository,
                    transfer::DynCardStatsTransferByCardRepository,
                    withdraw::DynCardStatsWithdrawByCardRepository,
                },
            },
            service::{
                command::DynCardCommandService,
                dashboard::DynCardDashboardService,
                query::DynCardQueryService,
                stats::{
                    balance::DynCardStatsBalanceService, topup::DynCardStatsTopupService,
                    transaction::DynCardStatsTransactionService,
                    transfer::DynCardStatsTransferService, withdraw::DynCardStatsWithdrawService,
                },
                statsbycard::{
                    balance::DynCardStatsBalanceByCardService,
                    topup::DynCardStatsTopupByCardService,
                    transaction::DynCardStatsTransactionByCardService,
                    transfer::DynCardStatsTransferByCardService,
                    withdraw::DynCardStatsWithdrawByCardService,
                },
            },
        },
        user::repository::query::DynUserQueryRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    context::shared_resources::SharedResources,
    observability::{CacheMetricsCore, TracingMetricsCore},
    repository::{
        card::{
            command::CardCommandRepository,
            dashboard::{
                balance::CardDashboardBalanceRepository, topup::CardDashboardTopupRepository,
                transaction::CardDashboardTransactionRepository,
                transfer::CardDashboardTransferRepository,
                withdraw::CardDashboardWithdrawRepository,
            },
            query::CardQueryRepository,
            stats::{
                balance::CardStatsBalanceRepository, topup::CardStatsTopupRepository,
                transaction::CardStatsTransactionRepository, transfer::CardStatsTransferRepository,
                withdraw::CardStatsWithdrawRepository,
            },
            statsbycard::{
                balance::CardStatsBalanceByCardRepository, topup::CardStatsTopupByCardRepository,
                transaction::CardStatsTransactionByCardRepository,
                transfer::CardStatsTransferByCardRepository,
                withdraw::CardStatsWithdrawByCardRepository,
            },
        },
        user::query::UserQueryRepository,
    },
    service::card::{
        command::{CardCommandService, CardCommandServiceDeps},
        dashboard::{CardDashboardService, CardDashboardServiceDeps},
        query::CardQueryService,
        stats::{
            balance::CardStatsBalanceService, topup::CardStatsTopupService,
            transaction::CardStatsTransactionService, transfer::CardStatsTransferService,
            withdraw::CardStatsWithdrawService,
        },
        statsbycard::{
            balance::CardStatsBalanceByCardService, topup::CardStatsTopupByCardService,
            transaction::CardStatsTransactionByCardService,
            transfer::CardStatsTransferByCardService, withdraw::CardStatsWithdrawByCardService,
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
    pub card_query: DynCardQueryService,
    pub card_command: DynCardCommandService,
    pub card_dashboard: DynCardDashboardService,

    pub stats_balance: DynCardStatsBalanceService,
    pub stats_topup: DynCardStatsTopupService,
    pub stats_transaction: DynCardStatsTransactionService,
    pub stats_transfer: DynCardStatsTransferService,
    pub stats_withdraw: DynCardStatsWithdrawService,

    pub stats_bycard_balance: DynCardStatsBalanceByCardService,
    pub stats_bycard_topup: DynCardStatsTopupByCardService,
    pub stats_bycard_transaction: DynCardStatsTransactionByCardService,
    pub stats_bycard_transfer: DynCardStatsTransferByCardService,
    pub stats_bycard_withdraw: DynCardStatsWithdrawByCardService,

    pub cache_store: Arc<CacheStore>,
    pub request_limiter: Arc<Semaphore>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("DependenciesInject");

        debug
            .field("card_query", &"DynCardQueryService")
            .field("card_command", &"DynCardCommandService")
            .field("card_dashboard", &"DynCardDashboardService")
            .field("stats_balance", &"DynCardStatsBalanceService")
            .field("stats_topup", &"DynCardStatsTopupService")
            .field("stats_transaction", &"DynCardStatsTransactionService")
            .field("stats_transfer", &"DynCardStatsTransferService")
            .field("stats_withdraw", &"DynCardStatsWithdrawService")
            .field("stats_bycard_balance", &"DynCardStatsBalanceByCardService")
            .field("stats_bycard_topup", &"DynCardStatsTopupByCardService")
            .field(
                "stats_bycard_transaction",
                &"DynCardStatsTransactionByCardService",
            )
            .field(
                "stats_bycard_transfer",
                &"DynCardStatsTransferByCardService",
            )
            .field(
                "stats_bycard_withdraw",
                &"DynCardStatsWithdrawByCardService",
            );

        debug.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_metrics = Arc::new(CacheMetricsCore::new("cache"));
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone(), cache_metrics));

        let tracing_metrics =
            Arc::new(TracingMetricsCore::new("card-service").context("failed initialize tracing")?);

        let user_query_repo =
            Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let card_command_repo =
            Arc::new(CardCommandRepository::new(db.clone())) as DynCardCommandRepository;

        let shared = SharedResources {
            tracing_metrics,
            cache_store,
        };

        let card_query = Arc::new(
            CardQueryService::new(card_query_repo.clone(), &shared)
                .context("failed initialize card query")?,
        ) as DynCardQueryService;

        let card_command_deps = CardCommandServiceDeps {
            user_query: user_query_repo.clone(),
            query: card_query_repo.clone(),
            command: card_command_repo.clone(),
        };
        let card_command = Arc::new(
            CardCommandService::new(card_command_deps, &shared)
                .context("failed initialize card command")?,
        ) as DynCardCommandService;

        let card_dashboard_deps = CardDashboardServiceDeps {
            balance: Arc::new(CardDashboardBalanceRepository::new(db.clone()))
                as DynCardDashboardBalanceRepository,
            topup: Arc::new(CardDashboardTopupRepository::new(db.clone()))
                as DynCardDashboardTopupRepository,
            transaction: Arc::new(CardDashboardTransactionRepository::new(db.clone()))
                as DynCardDashboardTransactionRepository,
            transfer: Arc::new(CardDashboardTransferRepository::new(db.clone()))
                as DynCardDashboardTransferRepository,
            withdraw: Arc::new(CardDashboardWithdrawRepository::new(db.clone()))
                as DynCardDashboardWithdrawRepository,
        };
        let card_dashboard = Arc::new(
            CardDashboardService::new(card_dashboard_deps, &shared)
                .context("failed initialize card dashboard")?,
        ) as DynCardDashboardService;

        // Stats

        let stats_balance = Arc::new(
            CardStatsBalanceService::new(
                Arc::new(CardStatsBalanceRepository::new(db.clone()))
                    as DynCardStatsBalanceRepository,
                &shared,
            )
            .context("failed initialize card stats balance")?,
        ) as DynCardStatsBalanceService;

        let stats_topup = Arc::new(
            CardStatsTopupService::new(
                Arc::new(CardStatsTopupRepository::new(db.clone())) as DynCardStatsTopupRepository,
                &shared,
            )
            .context("failed initialize card stats topup")?,
        ) as DynCardStatsTopupService;

        let stats_transaction = Arc::new(
            CardStatsTransactionService::new(
                Arc::new(CardStatsTransactionRepository::new(db.clone()))
                    as DynCardStatsTransactionRepository,
                &shared,
            )
            .context("failed initialize card stats transaction")?,
        ) as DynCardStatsTransactionService;

        let stats_transfer = Arc::new(
            CardStatsTransferService::new(
                Arc::new(CardStatsTransferRepository::new(db.clone()))
                    as DynCardStatsTransferRepository,
                &shared,
            )
            .context("failed initialize card stats transfer")?,
        ) as DynCardStatsTransferService;

        let stats_withdraw = Arc::new(
            CardStatsWithdrawService::new(
                Arc::new(CardStatsWithdrawRepository::new(db.clone()))
                    as DynCardStatsWithdrawRepository,
                &shared,
            )
            .context("failed initialize card stats withdraw")?,
        ) as DynCardStatsWithdrawService;

        // Stats By Card
        let stats_bycard_balance = Arc::new(
            CardStatsBalanceByCardService::new(
                Arc::new(CardStatsBalanceByCardRepository::new(db.clone()))
                    as DynCardStatsBalanceByCardRepository,
                &shared,
            )
            .context("Failed to initialize card stats balance service")?,
        ) as DynCardStatsBalanceByCardService;

        let stats_bycard_topup = Arc::new(
            CardStatsTopupByCardService::new(
                Arc::new(CardStatsTopupByCardRepository::new(db.clone()))
                    as DynCardStatsTopupByCardRepository,
                &shared,
            )
            .context("Failed to initialize card stats topup service")?,
        ) as DynCardStatsTopupByCardService;

        let stats_bycard_transaction = Arc::new(
            CardStatsTransactionByCardService::new(
                Arc::new(CardStatsTransactionByCardRepository::new(db.clone()))
                    as DynCardStatsTransactionByCardRepository,
                &shared,
            )
            .context("Failed to initialize card stats transaction")?,
        ) as DynCardStatsTransactionByCardService;

        let stats_bycard_transfer = Arc::new(
            CardStatsTransferByCardService::new(
                Arc::new(CardStatsTransferByCardRepository::new(db.clone()))
                    as DynCardStatsTransferByCardRepository,
                &shared,
            )
            .context("Failed to initialize card stats transfer")?,
        ) as DynCardStatsTransferByCardService;

        let stats_bycard_withdraw = Arc::new(
            CardStatsWithdrawByCardService::new(
                Arc::new(CardStatsWithdrawByCardRepository::new(db.clone()))
                    as DynCardStatsWithdrawByCardRepository,
                &shared,
            )
            .context("Failed to initialize card stats withdraw")?,
        ) as DynCardStatsWithdrawByCardService;

        Self::spawn_monitoring_task(Arc::clone(&shared.cache_store));
        Self::spawn_cleanup_task(Arc::clone(&shared.cache_store));

        Ok(Self {
            card_query,
            card_command,
            card_dashboard,
            stats_balance,
            stats_topup,
            stats_transaction,
            stats_transfer,
            stats_withdraw,
            stats_bycard_balance,
            stats_bycard_topup,
            stats_bycard_transaction,
            stats_bycard_transfer,
            stats_bycard_withdraw,
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
