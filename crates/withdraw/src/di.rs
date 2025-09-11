use anyhow::Result;
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
    config::ConnectionPool,
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
        command::WithdrawCommandService,
        query::WithdrawQueryService,
        stats::{amount::WithdrawStatsAmountService, status::WithdrawStatsStatusService},
        statsbycard::{
            amount::WithdrawStatsAmountByCardService, status::WithdrawStatsStatusByCardService,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct WithdrawCommandDeps {
    pub repo: DynWithdrawCommandRepository,
    pub service: DynWithdrawCommandService,
}

impl WithdrawCommandDeps {
    pub async fn new(db: ConnectionPool, query: DynWithdrawQueryRepository) -> Result<Self> {
        let saldo_query =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let card_query = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let repo =
            Arc::new(WithdrawCommandRepository::new(db.clone())) as DynWithdrawCommandRepository;
        let service = Arc::new(
            WithdrawCommandService::new(
                query.clone(),
                repo.clone(),
                card_query.clone(),
                saldo_query.clone(),
                saldo_command.clone(),
            )
            .await,
        ) as DynWithdrawCommandService;

        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct WithdrawQueryDeps {
    pub service: DynWithdrawQueryService,
}

impl WithdrawQueryDeps {
    pub async fn new(query: DynWithdrawQueryRepository) -> Result<Self> {
        let service =
            Arc::new(WithdrawQueryService::new(query.clone()).await) as DynWithdrawQueryService;

        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct WithdrawStatsDeps {
    pub amount_repo: DynWithdrawStatsAmountRepository,
    pub status_repo: DynWithdrawStatsStatusRepository,
    pub amount_service: DynWithdrawStatsAmountService,
    pub status_service: DynWithdrawStatsStatusService,
}

impl WithdrawStatsDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(WithdrawStatsAmountRepository::new(db.clone()))
            as DynWithdrawStatsAmountRepository;
        let status_repo = Arc::new(WithdrawStatsStatusRepository::new(db.clone()))
            as DynWithdrawStatsStatusRepository;

        let amount_service = Arc::new(WithdrawStatsAmountService::new(amount_repo.clone()).await)
            as DynWithdrawStatsAmountService;
        let status_service = Arc::new(WithdrawStatsStatusService::new(status_repo.clone()).await)
            as DynWithdrawStatsStatusService;

        Ok(Self {
            amount_repo,
            status_repo,
            amount_service,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct WithdrawStatsByCardDeps {
    pub amount_repo: DynWithdrawStatsAmountByCardRepository,
    pub status_repo: DynWithdrawStatsStatusByCardRepository,
    pub amount_service: DynWithdrawStatsAmountByCardService,
    pub status_service: DynWithdrawStatsStatusByCardService,
}

impl WithdrawStatsByCardDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(WithdrawStatsAmountByCardRepository::new(db.clone()))
            as DynWithdrawStatsAmountByCardRepository;
        let status_repo = Arc::new(WithdrawStatsStatusByCardRepository::new(db.clone()))
            as DynWithdrawStatsStatusByCardRepository;

        let amount_service =
            Arc::new(WithdrawStatsAmountByCardService::new(amount_repo.clone()).await)
                as DynWithdrawStatsAmountByCardService;
        let status_service =
            Arc::new(WithdrawStatsStatusByCardService::new(status_repo.clone()).await)
                as DynWithdrawStatsStatusByCardService;

        Ok(Self {
            amount_repo,
            status_repo,
            amount_service,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub withdraw_command: WithdrawCommandDeps,
    pub withdraw_query: WithdrawQueryDeps,
    pub withdraw_stats: WithdrawStatsDeps,
    pub withdraw_stats_by_card: WithdrawStatsByCardDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let query =
            Arc::new(WithdrawQueryRepository::new(db.clone())) as DynWithdrawQueryRepository;

        let withdraw_query = WithdrawQueryDeps::new(query.clone()).await?;

        let withdraw_command = WithdrawCommandDeps::new(db.clone(), query.clone()).await?;

        let withdraw_stats = WithdrawStatsDeps::new(db.clone()).await?;
        let withdraw_stats_by_card = WithdrawStatsByCardDeps::new(db.clone()).await?;

        Ok(Self {
            withdraw_command,
            withdraw_query,
            withdraw_stats,
            withdraw_stats_by_card,
        })
    }
}
