use anyhow::{Context, Result};
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
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
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
        command::{TopupCommandService, TopupCommandServiceDeps},
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
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub topup_command: DynTopupCommandService,
    pub topup_query: DynTopupQueryService,
    pub topup_stats_amount: DynTopupStatsAmountService,
    pub topup_stats_method: DynTopupStatsMethodService,
    pub topup_stats_status: DynTopupStatsStatusService,
    pub topup_stats_amount_by_card: DynTopupStatsAmountByCardService,
    pub topup_stats_method_by_card: DynTopupStatsMethodByCardService,
    pub topup_stats_status_by_card: DynTopupStatsStatusByCardService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("DependenciesInject");

        debug_struct
            .field("topup_command", &"DynTopupCommandService")
            .field("topup_query", &"DynTopupQueryService")
            .field("topup_stats_amount", &"DynTopupStatsAmountService")
            .field("topup_stats_method", &"DynTopupStatsMethodService")
            .field("topup_stats_status", &"DynTopupStatsStatusService")
            .field(
                "topup_stats_amount_by_card",
                &"DynTopupStatsAmountByCardService",
            )
            .field(
                "topup_stats_method_by_card",
                &"DynTopupStatsMethodByCardService",
            )
            .field(
                "topup_stats_status_by_card",
                &"DynTopupStatsStatusByCardService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let topup_query_repo =
            Arc::new(TopupQueryRepository::new(db.clone())) as DynTopupQueryRepository;
        let topup_query = Arc::new(
            TopupQueryService::new(topup_query_repo.clone(), cache_store.clone())
                .context("failed to initialize topup query service")?,
        ) as DynTopupQueryService;

        let topup_command_repo =
            Arc::new(TopupCommandRepository::new(db.clone())) as DynTopupCommandRepository;
        let card_query_repo =
            Arc::new(CardQueryRepository::new(db.clone())) as DynCardQueryRepository;
        let saldo_query_repo =
            Arc::new(SaldoQueryRepository::new(db.clone())) as DynSaldoQueryRepository;
        let saldo_command_repo =
            Arc::new(SaldoCommandRepository::new(db.clone())) as DynSaldoCommandRepository;

        let command_deps = TopupCommandServiceDeps {
            card_query: card_query_repo,
            saldo_query: saldo_query_repo,
            saldo_command: saldo_command_repo,
            query: topup_query_repo.clone(),
            cache_store: cache_store.clone(),
            command: topup_command_repo.clone(),
        };
        let topup_command = Arc::new(
            TopupCommandService::new(command_deps)
                .context("failed to initialize topup command service")?,
        ) as DynTopupCommandService;

        let amount_repo =
            Arc::new(TopupStatsAmountRepository::new(db.clone())) as DynTopupStatsAmountRepository;
        let topup_stats_amount = Arc::new(
            TopupStatsAmountService::new(amount_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats amount service")?,
        ) as DynTopupStatsAmountService;

        let method_repo =
            Arc::new(TopupStatsMethodRepository::new(db.clone())) as DynTopupStatsMethodRepository;
        let topup_stats_method = Arc::new(
            TopupStatsMethodService::new(method_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats method service")?,
        ) as DynTopupStatsMethodService;

        let status_repo =
            Arc::new(TopupStatsStatusRepository::new(db.clone())) as DynTopupStatsStatusRepository;
        let topup_stats_status = Arc::new(
            TopupStatsStatusService::new(status_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats status service")?,
        ) as DynTopupStatsStatusService;

        let amount_by_card_repo = Arc::new(TopupStatsAmountByCardRepository::new(db.clone()))
            as DynTopupStatsAmountByCardRepository;
        let topup_stats_amount_by_card = Arc::new(
            TopupStatsAmountByCardService::new(amount_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats amount by card service")?,
        ) as DynTopupStatsAmountByCardService;

        let method_by_card_repo = Arc::new(TopupStatsMethodByCardRepository::new(db.clone()))
            as DynTopupStatsMethodByCardRepository;
        let topup_stats_method_by_card = Arc::new(
            TopupStatsMethodByCardService::new(method_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats method by card service")?,
        ) as DynTopupStatsMethodByCardService;

        let status_by_card_repo = Arc::new(TopupStatsStatusByCardRepository::new(db.clone()))
            as DynTopupStatsStatusByCardRepository;
        let topup_stats_status_by_card = Arc::new(
            TopupStatsStatusByCardService::new(status_by_card_repo.clone(), cache_store.clone())
                .context("failed to initialize topup stats status by card service")?,
        ) as DynTopupStatsStatusByCardService;

        Ok(Self {
            topup_command,
            topup_query,
            topup_stats_amount,
            topup_stats_method,
            topup_stats_status,
            topup_stats_amount_by_card,
            topup_stats_method_by_card,
            topup_stats_status_by_card,
        })
    }
}
