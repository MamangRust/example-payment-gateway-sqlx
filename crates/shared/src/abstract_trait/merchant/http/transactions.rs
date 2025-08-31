use crate::{
    domain::{
        requests::merchant::{
            FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById,
        },
        responses::{ApiResponsePagination, MerchantTransactionResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantTransactionService = Arc<dyn MerchantTransactionServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantTransactionServiceTrait {
    async fn find_all_transactiions(
        &self,
        request: &FindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError>;

    async fn find_all_transactiions_by_api_key(
        &self,
        request: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError>;
    async fn find_all_transactiions_by_id(
        &self,
        request: &FindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, ServiceError>;
}
