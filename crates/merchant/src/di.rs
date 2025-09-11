use std::sync::Arc;

use anyhow::Result;
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
    config::ConnectionPool,
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

#[derive(Clone)]
pub struct MerchantQueryDeps {
    pub repo: DynMerchantQueryRepository,
    pub service: DynMerchantQueryService,
}

impl MerchantQueryDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let repo = Arc::new(MerchantQueryRepository::new(db.clone())) as DynMerchantQueryRepository;
        let service =
            Arc::new(MerchantQueryService::new(repo.clone()).await) as DynMerchantQueryService;
        Self { repo, service }
    }
}

#[derive(Clone)]
pub struct MerchantTransactionDeps {
    pub repo: DynMerchantTransactionRepository,
    pub service: DynMerchantTransactionService,
}

impl MerchantTransactionDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let repo = Arc::new(MerchantTransactionRepository::new(db.clone()))
            as DynMerchantTransactionRepository;
        let service = Arc::new(MerchantTransactionService::new(repo.clone()).await)
            as DynMerchantTransactionService;
        Self { repo, service }
    }
}

#[derive(Clone)]
pub struct MerchantCommandDeps {
    pub repo: DynMerchantCommandRepository,
    pub service: DynMerchantCommandService,
}

impl MerchantCommandDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let user_repo = Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;

        let repo =
            Arc::new(MerchantCommandRepository::new(db.clone())) as DynMerchantCommandRepository;
        let service = Arc::new(MerchantCommandService::new(repo.clone(), user_repo.clone()).await)
            as DynMerchantCommandService;
        Self { repo, service }
    }
}

#[derive(Clone)]
pub struct MerchantStatsDeps {
    pub amount: DynMerchantStatsAmountService,
    pub method: DynMerchantStatsMethodService,
    pub total: DynMerchantStatsTotalAmountService,
}

impl MerchantStatsDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let amount_repo = Arc::new(MerchantStatsAmountRepository::new(db.clone()))
            as DynMerchantStatsAmountRepository;
        let amount = Arc::new(MerchantStatsAmountService::new(amount_repo.clone()).await)
            as DynMerchantStatsAmountService;

        let method_repo = Arc::new(MerchantStatsMethodRepository::new(db.clone()))
            as DynMerchantStatsMethodRepository;
        let method = Arc::new(MerchantStatsMethodService::new(method_repo.clone()).await)
            as DynMerchantStatsMethodService;

        let total_repo = Arc::new(MerchantStatsTotalAmountRepository::new(db.clone()))
            as DynMerchantStatsTotalAmountRepository;
        let total = Arc::new(MerchantStatsTotalAmountService::new(total_repo.clone()).await)
            as DynMerchantStatsTotalAmountService;

        Self {
            amount,
            method,
            total,
        }
    }
}

#[derive(Clone)]
pub struct MerchantStatsByApiKeyDeps {
    pub amount: DynMerchantStatsAmountByApiKeyService,
    pub method: DynMerchantStatsMethodByApiKeyService,
    pub total: DynMerchantStatsTotalAmountByApiKeyService,
}

impl MerchantStatsByApiKeyDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let amount_repo = Arc::new(MerchantStatsAmountByApiKeyRepository::new(db.clone()))
            as DynMerchantStatsAmountByApiKeyRepository;
        let amount = Arc::new(MerchantStatsAmountByApiKeyService::new(amount_repo.clone()).await)
            as DynMerchantStatsAmountByApiKeyService;

        let method_repo = Arc::new(MerchantStatsMethodByApiKeyRepository::new(db.clone()))
            as DynMerchantStatsMethodByApiKeyRepository;
        let method = Arc::new(MerchantStatsMethodByApiKeyService::new(method_repo.clone()).await)
            as DynMerchantStatsMethodByApiKeyService;

        let total_repo = Arc::new(MerchantStatsTotalAmountByApiKeyRepository::new(db.clone()))
            as DynMerchantStatsTotalAmountByApiKeyRepository;
        let total = Arc::new(MerchantStatsTotalAmountByApiKeyService::new(total_repo.clone()).await)
            as DynMerchantStatsTotalAmountByApiKeyService;

        Self {
            amount,
            method,
            total,
        }
    }
}

#[derive(Clone)]
pub struct MerchantStatsByMerchantDeps {
    pub amount: DynMerchantStatsAmountByMerchantService,
    pub method: DynMerchantStatsMethodByMerchantService,
    pub total: DynMerchantStatsTotalAmountByMerchantService,
}

impl MerchantStatsByMerchantDeps {
    pub async fn new(db: ConnectionPool) -> Self {
        let amount_repo = Arc::new(MerchantStatsAmountByMerchantRepository::new(db.clone()))
            as DynMerchantStatsAmountByMerchantRepository;
        let amount = Arc::new(MerchantStatsAmountByMerchantService::new(amount_repo.clone()).await)
            as DynMerchantStatsAmountByMerchantService;

        let method_repo = Arc::new(MerchantStatsMethodByMerchantRepository::new(db.clone()))
            as DynMerchantStatsMethodByMerchantRepository;
        let method = Arc::new(MerchantStatsMethodByMerchantService::new(method_repo.clone()).await)
            as DynMerchantStatsMethodByMerchantService;

        let total_repo = Arc::new(MerchantStatsTotalAmountByMerchantRepository::new(
            db.clone(),
        )) as DynMerchantStatsTotalAmountByMerchantRepository;
        let total =
            Arc::new(MerchantStatsTotalAmountByMerchantService::new(total_repo.clone()).await)
                as DynMerchantStatsTotalAmountByMerchantService;

        Self {
            amount,
            method,
            total,
        }
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub merchant_command: MerchantCommandDeps,
    pub merchant_query: MerchantQueryDeps,
    pub merchant_transaction: MerchantTransactionDeps,
    pub merchant_stats: MerchantStatsDeps,
    pub merchant_stats_by_apikey: MerchantStatsByApiKeyDeps,
    pub merchant_stats_by_merchant: MerchantStatsByMerchantDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let merchant_command = MerchantCommandDeps::new(db.clone()).await;
        let merchant_query = MerchantQueryDeps::new(db.clone()).await;
        let merchant_transaction = MerchantTransactionDeps::new(db.clone()).await;
        let merchant_stats = MerchantStatsDeps::new(db.clone()).await;
        let merchant_stats_by_apikey = MerchantStatsByApiKeyDeps::new(db.clone()).await;
        let merchant_stats_by_merchant = MerchantStatsByMerchantDeps::new(db.clone()).await;

        Ok(Self {
            merchant_command,
            merchant_query,
            merchant_transaction,
            merchant_stats,
            merchant_stats_by_apikey,
            merchant_stats_by_merchant,
        })
    }
}
