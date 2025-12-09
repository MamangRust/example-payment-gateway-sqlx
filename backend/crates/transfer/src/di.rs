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
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub transfer_command: DynTransferCommandService,
    pub transfer_query: DynTransferQueryService,
    pub transfer_stats_amount: DynTransferStatsAmountService,
    pub transfer_stats_status: DynTransferStatsStatusService,
    pub transfer_stats_amount_by_card: DynTransferStatsAmountByCardService,
    pub transfer_stats_status_by_card: DynTransferStatsStatusByCardService,
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
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let transfer_query_repo =
            Arc::new(TransferQueryRepository::new(db.clone())) as DynTransferQueryRepository;

        let transfer_query = Arc::new(
            TransferQueryService::new(transfer_query_repo.clone(), cache_store.clone())
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
            cache_store: cache_store.clone(),
        };
        let transfer_command = Arc::new(
            TransferCommandService::new(command_deps)
                .context("failed to initialize transfer command service")?,
        ) as DynTransferCommandService;

        let amount_repo = Arc::new(TransferStatsAmountRepository::new(db.clone()))
            as DynTransferStatsAmountRepository;
        let transfer_stats_amount = Arc::new(
            TransferStatsAmountService::new(amount_repo.clone(), cache_store.clone())
                .context("failed to initialize transfer stats amount service")?,
        ) as DynTransferStatsAmountService;

        let status_repo = Arc::new(TransferStatsStatusRepository::new(db.clone()))
            as DynTransferStatsStatusRepository;
        let transfer_stats_status = Arc::new(
            TransferStatsStatusService::new(status_repo.clone(), cache_store.clone())
                .context("failed to initialize transfer stats status service")?,
        ) as DynTransferStatsStatusService;

        let amount_by_card_repo = Arc::new(TransferStatsAmountByCardRepository::new(db.clone()))
            as DynTransferStatsAmountByCardRepository;
        let transfer_stats_amount_by_card = Arc::new(
            TransferStatsAmountByCardService::new(amount_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize transfer stats amount by card service")?,
        ) as DynTransferStatsAmountByCardService;

        let status_by_card_repo = Arc::new(TransferStatsStatusByCardRepository::new(db.clone()))
            as DynTransferStatsStatusByCardRepository;
        let transfer_stats_status_by_card = Arc::new(
            TransferStatsStatusByCardService::new(status_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize transfer stats status by card service")?,
        ) as DynTransferStatsStatusByCardService;

        Ok(Self {
            transfer_command,
            transfer_query,
            transfer_stats_amount,
            transfer_stats_status,
            transfer_stats_amount_by_card,
            transfer_stats_status_by_card,
        })
    }
}
