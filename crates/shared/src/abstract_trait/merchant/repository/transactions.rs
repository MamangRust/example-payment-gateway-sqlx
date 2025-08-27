use crate::{
    domain::requests::{
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
        request: FindAllMerchantTransactions,
    ) -> Result<Vec<MerchantTransactionsModel>, RepositoryError>;

    async fn find_all_transactiions_by_api_key(
        &self,
        request: FindAllMerchantTransactionsByApiKey,
    ) -> Result<Vec<MerchantTransactionsModel>, RepositoryError>;
    async fn find_all_transactiions_by_id(
        &self,
        request: FindAllMerchantTransactionsById,
    ) -> Result<Vec<MerchantTransactionsModel>, RepositoryError>;
}
