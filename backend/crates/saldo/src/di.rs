use anyhow::{Context, Result};
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
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    repository::{
        card::query::CardQueryRepository,
        saldo::{
            command::SaldoCommandRepository,
            query::SaldoQueryRepository,
            stats::{balance::SaldoBalanceRepository, total::SaldoTotalBalanceRepository},
        },
    },
    service::saldo::{
        command::{SaldoCommandService, SaldoCommandServiceDeps},
        query::SaldoQueryService,
        stats::{balance::SaldoBalanceService, total::SaldoTotalBalanceService},
    },
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub saldo_command: DynSaldoCommandService,
    pub saldo_query: DynSaldoQueryService,
    pub saldo_balance: DynSaldoBalanceService,
    pub saldo_total_balance: DynSaldoTotalBalanceService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("saldo_command", &"DynSaldoCommandService")
            .field("saldo_query", &"DynSaldoQueryService")
            .field("saldo_balance", &"DynSaldoBalanceService")
            .field("saldo_total_balance", &"DynSaldoTotalBalanceService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_query = Arc::new(
            SaldoQueryService::new(saldo_query_repo.clone(), cache_store.clone())
                .context("failed to initialize saldo query service")?,
        ) as DynSaldoQueryService;

        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let command_service_deps = SaldoCommandServiceDeps {
            card_query: card_query_repo,
            command: saldo_command_repo,
            cache_store: cache_store.clone(),
        };
        let saldo_command = Arc::new(
            SaldoCommandService::new(command_service_deps)
                .context("failed to initialize saldo command service")?,
        ) as DynSaldoCommandService;

        let balance_repo =
            Arc::new(SaldoBalanceRepository::new(db.clone())) as DynSaldoBalanceRepository;
        let saldo_balance = Arc::new(
            SaldoBalanceService::new(balance_repo.clone(), cache_store.clone())
                .context("failed to initialize saldo balance service")?,
        ) as DynSaldoBalanceService;

        let total_repo = Arc::new(SaldoTotalBalanceRepository::new(db.clone()))
            as DynSaldoTotalBalanceRepository;
        let saldo_total_balance = Arc::new(
            SaldoTotalBalanceService::new(total_repo.clone(), cache_store.clone())
                .context("failed to initialize saldo total balance service")?,
        ) as DynSaldoTotalBalanceService;

        Ok(Self {
            saldo_command,
            saldo_query,
            saldo_balance,
            saldo_total_balance,
        })
    }
}
