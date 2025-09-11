use anyhow::Result;
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
        command::TransferCommandService,
        query::TransferQueryService,
        stats::{amount::TransferStatsAmountService, status::TransferStatsStatusService},
        statsbycard::{
            amount::TransferStatsAmountByCardService, status::TransferStatsStatusByCardService,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct TransferCommandDeps {
    pub repo: DynTransferCommandRepository,
    pub service: DynTransferCommandService,
}

impl TransferCommandDeps {
    pub async fn new(db: ConnectionPool, query: DynTransferQueryRepository) -> Result<Self> {
        let repo =
            Arc::new(TransferCommandRepository::new(db.clone())) as DynTransferCommandRepository;
        let card_query = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let service = Arc::new(
            TransferCommandService::new(
                card_query.clone(),
                saldo_query.clone(),
                saldo_command.clone(),
                query.clone(),
                repo.clone(),
            )
            .await,
        ) as DynTransferCommandService;

        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct TransferQueryDeps {
    pub service: DynTransferQueryService,
}

impl TransferQueryDeps {
    pub async fn new(query: DynTransferQueryRepository) -> Result<Self> {
        let service =
            Arc::new(TransferQueryService::new(query.clone()).await) as DynTransferQueryService;
        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct TransferStatsDeps {
    pub amount_repo: DynTransferStatsAmountRepository,
    pub amount_service: DynTransferStatsAmountService,
    pub status_repo: DynTransferStatsStatusRepository,
    pub status_service: DynTransferStatsStatusService,
}

impl TransferStatsDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(TransferStatsAmountRepository::new(db.clone()))
            as DynTransferStatsAmountRepository;
        let amount_service = Arc::new(TransferStatsAmountService::new(amount_repo.clone()).await)
            as DynTransferStatsAmountService;

        let status_repo = Arc::new(TransferStatsStatusRepository::new(db.clone()))
            as DynTransferStatsStatusRepository;
        let status_service = Arc::new(TransferStatsStatusService::new(status_repo.clone()).await)
            as DynTransferStatsStatusService;

        Ok(Self {
            amount_repo,
            amount_service,
            status_repo,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct TransferStatsByCardDeps {
    pub amount_repo: DynTransferStatsAmountByCardRepository,
    pub amount_service: DynTransferStatsAmountByCardService,
    pub status_repo: DynTransferStatsStatusByCardRepository,
    pub status_service: DynTransferStatsStatusByCardService,
}

impl TransferStatsByCardDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(TransferStatsAmountByCardRepository::new(db.clone()))
            as DynTransferStatsAmountByCardRepository;
        let amount_service =
            Arc::new(TransferStatsAmountByCardService::new(amount_repo.clone()).await)
                as DynTransferStatsAmountByCardService;

        let status_repo = Arc::new(TransferStatsStatusByCardRepository::new(db.clone()))
            as DynTransferStatsStatusByCardRepository;
        let status_service =
            Arc::new(TransferStatsStatusByCardService::new(status_repo.clone()).await)
                as DynTransferStatsStatusByCardService;

        Ok(Self {
            amount_repo,
            amount_service,
            status_repo,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub transfer_command: TransferCommandDeps,
    pub transfer_query: TransferQueryDeps,
    pub transfer_stats: TransferStatsDeps,
    pub transfer_stats_bycard: TransferStatsByCardDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let query =
            Arc::new(TransferQueryRepository::new(db.clone())) as DynTransferQueryRepository;

        let transfer_command = TransferCommandDeps::new(db.clone(), query.clone()).await?;
        let transfer_query = TransferQueryDeps::new(query.clone()).await?;
        let transfer_stats = TransferStatsDeps::new(db.clone()).await?;
        let transfer_stats_bycard = TransferStatsByCardDeps::new(db.clone()).await?;

        Ok(Self {
            transfer_command,
            transfer_query,
            transfer_stats,
            transfer_stats_bycard,
        })
    }
}
