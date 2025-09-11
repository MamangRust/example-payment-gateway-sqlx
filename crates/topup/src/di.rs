use anyhow::Result;
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
    config::ConnectionPool,
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
        command::TopupCommandService,
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
use std::sync::Arc;

#[derive(Clone)]
pub struct TopupCommandDeps {
    pub repo: DynTopupCommandRepository,
    pub service: DynTopupCommandService,
}

impl TopupCommandDeps {
    pub async fn new(db: ConnectionPool, query: DynTopupQueryRepository) -> Result<Self> {
        let repo = Arc::new(TopupCommandRepository::new(db.clone())) as DynTopupCommandRepository;
        let card_query = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let service = Arc::new(
            TopupCommandService::new(
                card_query.clone(),
                saldo_query.clone(),
                saldo_command.clone(),
                query.clone(),
                repo.clone(),
            )
            .await,
        ) as DynTopupCommandService;
        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct TopupQueryDeps {
    pub repo: DynTopupQueryRepository,
    pub service: DynTopupQueryService,
}

impl TopupQueryDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let repo = Arc::new(TopupQueryRepository::new(db.clone())) as DynTopupQueryRepository;
        let service = Arc::new(TopupQueryService::new(repo.clone()).await) as DynTopupQueryService;
        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct TopupStatsDeps {
    pub amount_service: DynTopupStatsAmountService,
    pub method_service: DynTopupStatsMethodService,
    pub status_service: DynTopupStatsStatusService,
}

impl TopupStatsDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo =
            Arc::new(TopupStatsAmountRepository::new(db.clone())) as DynTopupStatsAmountRepository;
        let amount_service = Arc::new(TopupStatsAmountService::new(amount_repo.clone()).await)
            as DynTopupStatsAmountService;

        let method_repo =
            Arc::new(TopupStatsMethodRepository::new(db.clone())) as DynTopupStatsMethodRepository;
        let method_service = Arc::new(TopupStatsMethodService::new(method_repo.clone()).await)
            as DynTopupStatsMethodService;

        let status_repo =
            Arc::new(TopupStatsStatusRepository::new(db.clone())) as DynTopupStatsStatusRepository;
        let status_service = Arc::new(TopupStatsStatusService::new(status_repo.clone()).await)
            as DynTopupStatsStatusService;

        Ok(Self {
            amount_service,
            method_service,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct TopupStatsByCardDeps {
    pub amount_service: DynTopupStatsAmountByCardService,
    pub method_service: DynTopupStatsMethodByCardService,
    pub status_service: DynTopupStatsStatusByCardService,
}

impl TopupStatsByCardDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(TopupStatsAmountByCardRepository::new(db.clone()))
            as DynTopupStatsAmountByCardRepository;
        let amount_service = Arc::new(TopupStatsAmountByCardService::new(amount_repo.clone()).await)
            as DynTopupStatsAmountByCardService;

        let method_repo = Arc::new(TopupStatsMethodByCardRepository::new(db.clone()))
            as DynTopupStatsMethodByCardRepository;
        let method_service = Arc::new(TopupStatsMethodByCardService::new(method_repo.clone()).await)
            as DynTopupStatsMethodByCardService;

        let status_repo = Arc::new(TopupStatsStatusByCardRepository::new(db.clone()))
            as DynTopupStatsStatusByCardRepository;
        let status_service = Arc::new(TopupStatsStatusByCardService::new(status_repo.clone()).await)
            as DynTopupStatsStatusByCardService;

        Ok(Self {
            amount_service,
            method_service,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub topup_command: TopupCommandDeps,
    pub topup_query: TopupQueryDeps,
    pub topup_stats: TopupStatsDeps,
    pub topup_stats_bycard: TopupStatsByCardDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let query = Arc::new(TopupQueryRepository::new(db.clone())) as DynTopupQueryRepository;

        let topup_command = TopupCommandDeps::new(db.clone(), query.clone()).await?;
        let topup_query = TopupQueryDeps::new(db.clone()).await?;
        let topup_stats = TopupStatsDeps::new(db.clone()).await?;
        let topup_stats_bycard = TopupStatsByCardDeps::new(db.clone()).await?;

        Ok(Self {
            topup_command,
            topup_query,
            topup_stats,
            topup_stats_bycard,
        })
    }
}
