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
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub withdraw_command: DynWithdrawCommandService,
    pub withdraw_query: DynWithdrawQueryService,
    pub withdraw_stats_amount: DynWithdrawStatsAmountService,
    pub withdraw_stats_status: DynWithdrawStatsStatusService,
    pub withdraw_stats_amount_by_card: DynWithdrawStatsAmountByCardService,
    pub withdraw_stats_status_by_card: DynWithdrawStatsStatusByCardService,
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
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let withdraw_query_repo =
            Arc::new(WithdrawQueryRepository::new(db.clone())) as DynWithdrawQueryRepository;

        let withdraw_query = Arc::new(
            WithdrawQueryService::new(withdraw_query_repo.clone(), cache_store.clone())
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
            cache_store: cache_store.clone(),
        };
        let withdraw_command = Arc::new(
            WithdrawCommandService::new(command_deps)
                .context("failed to initialize withdraw command service")?,
        ) as DynWithdrawCommandService;

        let amount_repo = Arc::new(WithdrawStatsAmountRepository::new(db.clone()))
            as DynWithdrawStatsAmountRepository;
        let withdraw_stats_amount = Arc::new(
            WithdrawStatsAmountService::new(amount_repo.clone(), cache_store.clone())
                .context("failed to initialize withdraw stats amount service")?,
        ) as DynWithdrawStatsAmountService;

        let status_repo = Arc::new(WithdrawStatsStatusRepository::new(db.clone()))
            as DynWithdrawStatsStatusRepository;
        let withdraw_stats_status = Arc::new(
            WithdrawStatsStatusService::new(status_repo.clone(), cache_store.clone())
                .context("failed to initialize withdraw stats status service")?,
        ) as DynWithdrawStatsStatusService;

        let amount_by_card_repo = Arc::new(WithdrawStatsAmountByCardRepository::new(db.clone()))
            as DynWithdrawStatsAmountByCardRepository;
        let withdraw_stats_amount_by_card = Arc::new(
            WithdrawStatsAmountByCardService::new(amount_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize withdraw stats amount by card service")?,
        ) as DynWithdrawStatsAmountByCardService;

        let status_by_card_repo = Arc::new(WithdrawStatsStatusByCardRepository::new(db.clone()))
            as DynWithdrawStatsStatusByCardRepository;
        let withdraw_stats_status_by_card = Arc::new(
            WithdrawStatsStatusByCardService::new(status_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize withdraw stats status by card service")?,
        ) as DynWithdrawStatsStatusByCardService;

        Ok(Self {
            withdraw_command,
            withdraw_query,
            withdraw_stats_amount,
            withdraw_stats_status,
            withdraw_stats_amount_by_card,
            withdraw_stats_status_by_card,
        })
    }
}
