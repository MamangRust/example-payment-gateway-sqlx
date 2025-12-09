use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        merchant::{
            repository::{
                command::DynMerchantCommandRepository,
                query::DynMerchantQueryRepository,
                stats::{
                    amount::DynMerchantStatsAmountRepository,
                    method::DynMerchantStatsMethodRepository,
                    totalamount::DynMerchantStatsTotalAmountRepository,
                },
                statsbyapikey::{
                    amount::DynMerchantStatsAmountByApiKeyRepository,
                    method::DynMerchantStatsMethodByApiKeyRepository,
                    totalamount::DynMerchantStatsTotalAmountByApiKeyRepository,
                },
                statsbymerchant::{
                    amount::DynMerchantStatsAmountByMerchantRepository,
                    method::DynMerchantStatsMethodByMerchantRepository,
                    totalamount::DynMerchantStatsTotalAmountByMerchantRepository,
                },
                transactions::DynMerchantTransactionRepository,
            },
            service::{
                command::DynMerchantCommandService,
                query::DynMerchantQueryService,
                stats::{
                    amount::DynMerchantStatsAmountService, method::DynMerchantStatsMethodService,
                    totalamount::DynMerchantStatsTotalAmountService,
                },
                statsbyapikey::{
                    amount::DynMerchantStatsAmountByApiKeyService,
                    method::DynMerchantStatsMethodByApiKeyService,
                    totalamount::DynMerchantStatsTotalAmountByApiKeyService,
                },
                statsbymerchant::{
                    amount::DynMerchantStatsAmountByMerchantService,
                    method::DynMerchantStatsMethodByMerchantService,
                    totalamount::DynMerchantStatsTotalAmountByMerchantService,
                },
                transactions::DynMerchantTransactionService,
            },
        },
        user::repository::query::DynUserQueryRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    repository::{
        merchant::{
            command::MerchantCommandRepository,
            query::MerchantQueryRepository,
            stats::{
                amount::MerchantStatsAmountRepository, method::MerchantStatsMethodRepository,
                totalamount::MerchantStatsTotalAmountRepository,
            },
            statsbyapikey::{
                amount::MerchantStatsAmountByApiKeyRepository,
                method::MerchantStatsMethodByApiKeyRepository,
                totalamount::MerchantStatsTotalAmountByApiKeyRepository,
            },
            statsbymerchant::{
                amount::MerchantStatsAmountByMerchantRepository,
                method::MerchantStatsMethodByMerchantRepository,
                totalamount::MerchantStatsTotalAmountByMerchantRepository,
            },
            transactions::MerchantTransactionRepository,
        },
        user::query::UserQueryRepository,
    },
    service::merchant::{
        command::MerchantCommandService,
        query::MerchantQueryService,
        stats::{
            amount::MerchantStatsAmountService, method::MerchantStatsMethodService,
            totalamount::MerchantStatsTotalAmountService,
        },
        statsbyapikey::{
            amount::MerchantStatsAmountByApiKeyService, method::MerchantStatsMethodByApiKeyService,
            totalamount::MerchantStatsTotalAmountByApiKeyService,
        },
        statsbymerchant::{
            amount::MerchantStatsAmountByMerchantService,
            method::MerchantStatsMethodByMerchantService,
            totalamount::MerchantStatsTotalAmountByMerchantService,
        },
        transactions::MerchantTransactionService,
    },
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub merchant_query: DynMerchantQueryService,
    pub merchant_transaction: DynMerchantTransactionService,
    pub merchant_command: DynMerchantCommandService,

    // stats
    pub merchant_stats_amount: DynMerchantStatsAmountService,
    pub merchant_stats_method: DynMerchantStatsMethodService,
    pub merchant_stats_total_amount: DynMerchantStatsTotalAmountService,

    // stats by api key
    pub merchant_stats_amount_by_apikey: DynMerchantStatsAmountByApiKeyService,
    pub merchant_stats_method_by_apikey: DynMerchantStatsMethodByApiKeyService,
    pub merchant_stats_total_amount_by_apikey: DynMerchantStatsTotalAmountByApiKeyService,

    // stats by merchant
    pub merchant_stats_amount_by_merchant: DynMerchantStatsAmountByMerchantService,
    pub merchant_stats_method_by_merchant: DynMerchantStatsMethodByMerchantService,
    pub merchant_stats_total_amount_by_merchant: DynMerchantStatsTotalAmountByMerchantService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Dependencies");

        // Query
        debug_struct.field("merchant_query", &"DynMerchantQueryService");

        // Transaction
        debug_struct.field("merchant_transaction", &"DynMerchantTransactionService");

        // Command
        debug_struct.field("merchant_command", &"DynMerchantCommandService");

        // Stats
        debug_struct
            .field(
                "merchant_stats_amount_repo",
                &"DynMerchantStatsAmountRepository",
            )
            .field("merchant_stats_amount", &"DynMerchantStatsAmountService")
            .field("merchant_stats_method", &"DynMerchantStatsMethodService")
            .field(
                "merchant_stats_total_amount",
                &"DynMerchantStatsTotalAmountService",
            );

        // Stats By ApiKey
        debug_struct
            .field(
                "merchant_stats_amount_by_apikey",
                &"DynMerchantStatsAmountByApiKeyService",
            )
            .field(
                "merchant_stats_method_by_apikey",
                &"DynMerchantStatsMethodByApiKeyService",
            )
            .field(
                "merchant_stats_total_amount_by_apikey",
                &"DynMerchantStatsTotalAmountByApiKeyService",
            );

        // Stats By Merchant
        debug_struct
            .field(
                "merchant_stats_amount_by_merchant",
                &"DynMerchantStatsAmountByMerchantService",
            )
            .field(
                "merchant_stats_method_by_merchant",
                &"DynMerchantStatsMethodByMerchantService",
            )
            .field(
                "merchant_stats_total_amount_by_merchant",
                &"DynMerchantStatsTotalAmountByMerchantService",
            );

        debug_struct.finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        // query
        let merchant_query_repo =
            Arc::new(MerchantQueryRepository::new(db.clone())) as DynMerchantQueryRepository;
        let merchant_query = Arc::new(
            MerchantQueryService::new(merchant_query_repo.clone(), cache_store.clone())
                .context("failed to initialize merchant query service")?,
        ) as DynMerchantQueryService;

        // transaction
        let merchant_transaction_repo = Arc::new(MerchantTransactionRepository::new(db.clone()))
            as DynMerchantTransactionRepository;
        let merchant_transaction = Arc::new(
            MerchantTransactionService::new(merchant_transaction_repo.clone(), cache_store.clone())
                .context("failed to initialize merchant transaction service")?,
        ) as DynMerchantTransactionService;

        // command
        let user_query_repo =
            Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;
        let merchant_command_repo =
            Arc::new(MerchantCommandRepository::new(db.clone())) as DynMerchantCommandRepository;
        let merchant_command = Arc::new(
            MerchantCommandService::new(
                merchant_command_repo.clone(),
                user_query_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant command service")?,
        ) as DynMerchantCommandService;

        // stats
        let merchant_stats_amount_repo = Arc::new(MerchantStatsAmountRepository::new(db.clone()))
            as DynMerchantStatsAmountRepository;
        let merchant_stats_amount = Arc::new(
            MerchantStatsAmountService::new(
                merchant_stats_amount_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats amount service")?,
        ) as DynMerchantStatsAmountService;

        let merchant_stats_method_repo = Arc::new(MerchantStatsMethodRepository::new(db.clone()))
            as DynMerchantStatsMethodRepository;
        let merchant_stats_method = Arc::new(
            MerchantStatsMethodService::new(
                merchant_stats_method_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats method service")?,
        ) as DynMerchantStatsMethodService;

        let merchant_stats_total_amount_repo =
            Arc::new(MerchantStatsTotalAmountRepository::new(db.clone()))
                as DynMerchantStatsTotalAmountRepository;
        let merchant_stats_total_amount = Arc::new(
            MerchantStatsTotalAmountService::new(
                merchant_stats_total_amount_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats total amount service")?,
        ) as DynMerchantStatsTotalAmountService;

        // stats by apikey
        let merchant_stats_amount_by_apikey_repo =
            Arc::new(MerchantStatsAmountByApiKeyRepository::new(db.clone()))
                as DynMerchantStatsAmountByApiKeyRepository;
        let merchant_stats_amount_by_apikey = Arc::new(
            MerchantStatsAmountByApiKeyService::new(
                merchant_stats_amount_by_apikey_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats amount by apikey service")?,
        ) as DynMerchantStatsAmountByApiKeyService;

        let merchant_stats_method_by_apikey_repo =
            Arc::new(MerchantStatsMethodByApiKeyRepository::new(db.clone()))
                as DynMerchantStatsMethodByApiKeyRepository;
        let merchant_stats_method_by_apikey = Arc::new(
            MerchantStatsMethodByApiKeyService::new(
                merchant_stats_method_by_apikey_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats method by apikey service")?,
        ) as DynMerchantStatsMethodByApiKeyService;

        let merchant_stats_total_amount_by_apikey_repo =
            Arc::new(MerchantStatsTotalAmountByApiKeyRepository::new(db.clone()))
                as DynMerchantStatsTotalAmountByApiKeyRepository;
        let merchant_stats_total_amount_by_apikey = Arc::new(
            MerchantStatsTotalAmountByApiKeyService::new(
                merchant_stats_total_amount_by_apikey_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats total amount by apikey service")?,
        )
            as DynMerchantStatsTotalAmountByApiKeyService;

        // stats by merchant
        let merchant_stats_amount_by_merchant_repo =
            Arc::new(MerchantStatsAmountByMerchantRepository::new(db.clone()))
                as DynMerchantStatsAmountByMerchantRepository;
        let merchant_stats_amount_by_merchant = Arc::new(
            MerchantStatsAmountByMerchantService::new(
                merchant_stats_amount_by_merchant_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats amount by merchant service")?,
        )
            as DynMerchantStatsAmountByMerchantService;

        let merchant_stats_method_by_merchant_repo =
            Arc::new(MerchantStatsMethodByMerchantRepository::new(db.clone()))
                as DynMerchantStatsMethodByMerchantRepository;
        let merchant_stats_method_by_merchant = Arc::new(
            MerchantStatsMethodByMerchantService::new(
                merchant_stats_method_by_merchant_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats method by merchant service")?,
        )
            as DynMerchantStatsMethodByMerchantService;

        let merchant_stats_total_amount_by_merchant_repo = Arc::new(
            MerchantStatsTotalAmountByMerchantRepository::new(db.clone()),
        )
            as DynMerchantStatsTotalAmountByMerchantRepository;
        let merchant_stats_total_amount_by_merchant = Arc::new(
            MerchantStatsTotalAmountByMerchantService::new(
                merchant_stats_total_amount_by_merchant_repo.clone(),
                cache_store.clone(),
            )
            .context("failed to initialize merchant stats total amount by merchant service")?,
        )
            as DynMerchantStatsTotalAmountByMerchantService;

        Ok(Self {
            merchant_query,
            merchant_command,

            merchant_transaction,

            // stats
            merchant_stats_amount,
            merchant_stats_method,
            merchant_stats_total_amount,

            // stats by apikey
            merchant_stats_amount_by_apikey,
            merchant_stats_method_by_apikey,
            merchant_stats_total_amount_by_apikey,

            // stats by merchant
            merchant_stats_amount_by_merchant,
            merchant_stats_method_by_merchant,
            merchant_stats_total_amount_by_merchant,
        })
    }
}
