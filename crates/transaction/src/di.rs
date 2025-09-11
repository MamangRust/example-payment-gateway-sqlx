use anyhow::Result;
use shared::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        merchant::repository::query::DynMerchantQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        transaction::{
            repository::{
                command::DynTransactionCommandRepository,
                query::DynTransactionQueryRepository,
                stats::{
                    amount::DynTransactionStatsAmountRepository,
                    method::DynTransactionStatsMethodRepository,
                    status::DynTransactionStatsStatusRepository,
                },
                statsbycard::{
                    amount::DynTransactionStatsAmountByCardRepository,
                    method::DynTransactionStatsMethodByCardRepository,
                    status::DynTransactionStatsStatusByCardRepository,
                },
            },
            service::{
                command::DynTransactionCommandService,
                query::DynTransactionQueryService,
                stats::{
                    amount::DynTransactionStatsAmountService,
                    method::DynTransactionStatsMethodService,
                    status::DynTransactionStatsStatusService,
                },
                statsbycard::{
                    amount::DynTransactionStatsAmountByCardService,
                    method::DynTransactionStatsMethodByCardService,
                    status::DynTransactionStatsStatusByCardService,
                },
            },
        },
    },
    config::ConnectionPool,
    repository::{
        card::query::CardQueryRepository,
        merchant::query::MerchantQueryRepository,
        saldo::{command::SaldoCommandRepository, query::SaldoQueryRepository},
        transaction::{
            command::TransactionCommandRepository,
            query::TransactionQueryRepository,
            stats::{
                amount::TransactionStatsAmountRepository, method::TransactionStatsMethodRepository,
                status::TransactionStatsStatusRepository,
            },
            statsbycard::{
                amount::TransactionStatsAmountByCardRepository,
                method::TransactionStatsMethodByCardRepository,
                status::TransactionStatsStatusByCardRepository,
            },
        },
    },
    service::transaction::{
        command::TransactionCommandService,
        query::TransactionQueryService,
        stats::{
            amount::TransactionStatsAmountService, method::TransactionStatsMethodService,
            status::TransactionStatsStatusService,
        },
        statsbycard::{
            amount::TransactionStatsAmountByCardService,
            method::TransactionStatsMethodByCardService,
            status::TransactionStatsStatusByCardService,
        },
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct TransactionCommandDeps {
    pub repo: DynTransactionCommandRepository,
    pub service: DynTransactionCommandService,
}

impl TransactionCommandDeps {
    pub async fn new(db: ConnectionPool, query: DynTransactionQueryRepository) -> Result<Self> {
        let repo = Arc::new(TransactionCommandRepository::new(db.clone()))
            as DynTransactionCommandRepository;
        let merchant_query =
            Arc::new(MerchantQueryRepository::new(db.clone())) as DynMerchantQueryRepository;
        let saldo_query =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;
        let card_query = Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let service = Arc::new(
            TransactionCommandService::new(
                query.clone(),
                repo.clone(),
                merchant_query.clone(),
                saldo_query.clone(),
                saldo_command.clone(),
                card_query.clone(),
            )
            .await,
        ) as DynTransactionCommandService;
        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct TransactionQueryDeps {
    pub service: DynTransactionQueryService,
}

impl TransactionQueryDeps {
    pub async fn new(query: DynTransactionQueryRepository) -> Result<Self> {
        let service = Arc::new(TransactionQueryService::new(query.clone()).await)
            as DynTransactionQueryService;
        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct TransactionStatsDeps {
    pub amount_repo: DynTransactionStatsAmountRepository,
    pub amount_service: DynTransactionStatsAmountService,
    pub method_repo: DynTransactionStatsMethodRepository,
    pub method_service: DynTransactionStatsMethodService,
    pub status_repo: DynTransactionStatsStatusRepository,
    pub status_service: DynTransactionStatsStatusService,
}

impl TransactionStatsDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(TransactionStatsAmountRepository::new(db.clone()))
            as DynTransactionStatsAmountRepository;
        let amount_service = Arc::new(TransactionStatsAmountService::new(amount_repo.clone()).await)
            as DynTransactionStatsAmountService;

        let method_repo = Arc::new(TransactionStatsMethodRepository::new(db.clone()))
            as DynTransactionStatsMethodRepository;
        let method_service = Arc::new(TransactionStatsMethodService::new(method_repo.clone()).await)
            as DynTransactionStatsMethodService;

        let status_repo = Arc::new(TransactionStatsStatusRepository::new(db.clone()))
            as DynTransactionStatsStatusRepository;
        let status_service = Arc::new(TransactionStatsStatusService::new(status_repo.clone()).await)
            as DynTransactionStatsStatusService;

        Ok(Self {
            amount_repo,
            amount_service,
            method_repo,
            method_service,
            status_repo,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct TransactionStatsByCardDeps {
    pub amount_repo: DynTransactionStatsAmountByCardRepository,
    pub amount_service: DynTransactionStatsAmountByCardService,
    pub method_repo: DynTransactionStatsMethodByCardRepository,
    pub method_service: DynTransactionStatsMethodByCardService,
    pub status_repo: DynTransactionStatsStatusByCardRepository,
    pub status_service: DynTransactionStatsStatusByCardService,
}

impl TransactionStatsByCardDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let amount_repo = Arc::new(TransactionStatsAmountByCardRepository::new(db.clone()))
            as DynTransactionStatsAmountByCardRepository;
        let amount_service =
            Arc::new(TransactionStatsAmountByCardService::new(amount_repo.clone()).await)
                as DynTransactionStatsAmountByCardService;

        let method_repo = Arc::new(TransactionStatsMethodByCardRepository::new(db.clone()))
            as DynTransactionStatsMethodByCardRepository;
        let method_service =
            Arc::new(TransactionStatsMethodByCardService::new(method_repo.clone()).await)
                as DynTransactionStatsMethodByCardService;

        let status_repo = Arc::new(TransactionStatsStatusByCardRepository::new(db.clone()))
            as DynTransactionStatsStatusByCardRepository;
        let status_service =
            Arc::new(TransactionStatsStatusByCardService::new(status_repo.clone()).await)
                as DynTransactionStatsStatusByCardService;

        Ok(Self {
            amount_repo,
            amount_service,
            method_repo,
            method_service,
            status_repo,
            status_service,
        })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub transaction_command: TransactionCommandDeps,
    pub transaction_query: TransactionQueryDeps,
    pub transaction_stats: TransactionStatsDeps,
    pub transaction_stats_bycard: TransactionStatsByCardDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let query =
            Arc::new(TransactionQueryRepository::new(db.clone())) as DynTransactionQueryRepository;

        let transaction_command = TransactionCommandDeps::new(db.clone(), query.clone()).await?;
        let transaction_query = TransactionQueryDeps::new(query.clone()).await?;
        let transaction_stats = TransactionStatsDeps::new(db.clone()).await?;
        let transaction_stats_bycard = TransactionStatsByCardDeps::new(db.clone()).await?;

        Ok(Self {
            transaction_command,
            transaction_query,
            transaction_stats,
            transaction_stats_bycard,
        })
    }
}
