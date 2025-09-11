use anyhow::Result;
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
    config::ConnectionPool,
    repository::{
        card::query::CardQueryRepository,
        saldo::{
            command::SaldoCommandRepository,
            query::SaldoQueryRepository,
            stats::{balance::SaldoBalanceRepository, total::SaldoTotalBalanceRepository},
        },
    },
    service::saldo::{
        command::SaldoCommandService,
        query::SaldoQueryService,
        stats::{balance::SaldoBalanceService, total::SaldoTotalBalanceService},
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct SaldoCommandDeps {
    pub service: DynSaldoCommandService,
}

impl SaldoCommandDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let repo = Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;
        let card_repo = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let service = Arc::new(SaldoCommandService::new(repo.clone(), card_repo.clone()).await)
            as DynSaldoCommandService;
        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct SaldoQueryDeps {
    pub service: DynSaldoQueryService,
}

impl SaldoQueryDeps {
    pub async fn new(query: DynSaldoQueryRepository) -> Result<Self> {
        let service = Arc::new(SaldoQueryService::new(query.clone()).await) as DynSaldoQueryService;
        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct SaldoStatsDeps {
    pub balance_service: DynSaldoBalanceService,
    pub total_service: DynSaldoTotalBalanceService,
}

impl SaldoStatsDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let balance_repo =
            Arc::new(SaldoBalanceRepository::new(db.clone())) as DynSaldoBalanceRepository;
        let balance_service = Arc::new(SaldoBalanceService::new(balance_repo.clone()).await)
            as DynSaldoBalanceService;

        let total_repo = Arc::new(SaldoTotalBalanceRepository::new(db.clone()))
            as DynSaldoTotalBalanceRepository;
        let total_service = Arc::new(SaldoTotalBalanceService::new(total_repo.clone()).await)
            as DynSaldoTotalBalanceService;

        Ok(Self {
            balance_service,
            total_service,
        })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub saldo_command: SaldoCommandDeps,
    pub saldo_query: SaldoQueryDeps,
    pub saldo_stats: SaldoStatsDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let saldo_query =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;

        let saldo_command = SaldoCommandDeps::new(db.clone()).await?;
        let saldo_query = SaldoQueryDeps::new(saldo_query.clone()).await?;
        let saldo_stats = SaldoStatsDeps::new(db.clone()).await?;

        Ok(Self {
            saldo_command,
            saldo_query,
            saldo_stats,
        })
    }
}
