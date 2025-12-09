use anyhow::{Context, Result};
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
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
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
        command::{TransactionCommandService, TransactionCommandServiceDeps},
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
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub transaction_command: DynTransactionCommandService,
    pub transaction_query: DynTransactionQueryService,
    pub transaction_stats_amount: DynTransactionStatsAmountService,
    pub transaction_stats_method: DynTransactionStatsMethodService,
    pub transaction_stats_status: DynTransactionStatsStatusService,
    pub transaction_stats_amount_by_card: DynTransactionStatsAmountByCardService,
    pub transaction_stats_method_by_card: DynTransactionStatsMethodByCardService,
    pub transaction_stats_status_by_card: DynTransactionStatsStatusByCardService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DependenciesInject");

        debug_struct
            .field("transaction_command", &"DynTransactionCommandService")
            .field("transaction_query", &"DynTransactionQueryService")
            .field(
                "transaction_stats_amount",
                &"DynTransactionStatsAmountService",
            )
            .field(
                "transaction_stats_method",
                &"DynTransactionStatsMethodService",
            )
            .field(
                "transaction_stats_status",
                &"DynTransactionStatsStatusService",
            )
            .field(
                "transaction_stats_amount_by_card",
                &"DynTransactionStatsAmountByCardService",
            )
            .field(
                "transaction_stats_method_by_card",
                &"DynTransactionStatsMethodByCardService",
            )
            .field(
                "transaction_stats_status_by_card",
                &"DynTransactionStatsStatusByCardService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let transaction_query_repo =
            Arc::new(TransactionQueryRepository::new(db.clone())) as DynTransactionQueryRepository;

        let transaction_query = Arc::new(
            TransactionQueryService::new(transaction_query_repo.clone(), cache_store.clone())
                .context("failed to initialize transaction query service")?,
        ) as DynTransactionQueryService;

        let transaction_command_repo = Arc::new(TransactionCommandRepository::new(db.clone()))
            as DynTransactionCommandRepository;
        let merchant_query_repo =
            Arc::new(MerchantQueryRepository::new(db.clone())) as DynMerchantQueryRepository;
        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;

        let command_deps = TransactionCommandServiceDeps {
            query: transaction_query_repo.clone(),
            command: transaction_command_repo.clone(),
            merchant_query: merchant_query_repo,
            saldo_query: saldo_query_repo,
            saldo_command: saldo_command_repo,
            card_query: card_query_repo,
            cache_store: cache_store.clone(),
        };
        let transaction_command = Arc::new(
            TransactionCommandService::new(command_deps)
                .context("failed to initialize transaction command service")?,
        ) as DynTransactionCommandService;

        let amount_repo = Arc::new(TransactionStatsAmountRepository::new(db.clone()))
            as DynTransactionStatsAmountRepository;
        let transaction_stats_amount = Arc::new(
            TransactionStatsAmountService::new(amount_repo.clone(), cache_store.clone())
                .context("failed to initialize transaction stats amount service")?,
        ) as DynTransactionStatsAmountService;

        let method_repo = Arc::new(TransactionStatsMethodRepository::new(db.clone()))
            as DynTransactionStatsMethodRepository;
        let transaction_stats_method = Arc::new(
            TransactionStatsMethodService::new(method_repo.clone(), cache_store.clone())
                .context("failed to initialize transaction stats method service")?,
        ) as DynTransactionStatsMethodService;

        let status_repo = Arc::new(TransactionStatsStatusRepository::new(db.clone()))
            as DynTransactionStatsStatusRepository;
        let transaction_stats_status = Arc::new(
            TransactionStatsStatusService::new(status_repo.clone(), cache_store.clone())
                .context("failed to initialize transaction stats status service")?,
        ) as DynTransactionStatsStatusService;

        let amount_by_card_repo = Arc::new(TransactionStatsAmountByCardRepository::new(db.clone()))
            as DynTransactionStatsAmountByCardRepository;
        let transaction_stats_amount_by_card = Arc::new(
            TransactionStatsAmountByCardService::new(
                amount_by_card_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize transaction stats amount by card service")?,
        ) as DynTransactionStatsAmountByCardService;

        let method_by_card_repo = Arc::new(TransactionStatsMethodByCardRepository::new(db.clone()))
            as DynTransactionStatsMethodByCardRepository;
        let transaction_stats_method_by_card = Arc::new(
            TransactionStatsMethodByCardService::new(
                method_by_card_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize transaction stats method by card service")?,
        ) as DynTransactionStatsMethodByCardService;

        let status_by_card_repo = Arc::new(TransactionStatsStatusByCardRepository::new(db.clone()))
            as DynTransactionStatsStatusByCardRepository;
        let transaction_stats_status_by_card = Arc::new(
            TransactionStatsStatusByCardService::new(
                status_by_card_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize transaction stats status by card service")?,
        ) as DynTransactionStatsStatusByCardService;

        Ok(Self {
            transaction_command,
            transaction_query,
            transaction_stats_amount,
            transaction_stats_method,
            transaction_stats_status,
            transaction_stats_amount_by_card,
            transaction_stats_method_by_card,
            transaction_stats_status_by_card,
        })
    }
}
