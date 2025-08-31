use crate::{
    domain::requests::merchant::{
        FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
        FindAllMerchantTransactionsById,
    },
    errors::RepositoryError,
    model::merchant::MerchantTransactionsModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantTransactionRepository =
    Arc<dyn MerchantTransactionRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantTransactionRepositoryTrait {
    async fn find_all_transactiions(
        &self,
        req: &FindAllMerchantTransactions,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError>;

    async fn find_all_transactiions_by_api_key(
        &self,
        req: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError>;
    async fn find_all_transactiions_by_id(
        &self,
        req: &FindAllMerchantTransactionsById,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError>;
}
