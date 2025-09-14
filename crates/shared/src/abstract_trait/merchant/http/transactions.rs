use crate::{
    domain::{
        requests::merchant::{
            FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById,
        },
        responses::{ApiResponsePagination, MerchantTransactionResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantTransactionGrpcClientTrait {
    async fn find_all_transactiions(
        &self,
        request: &FindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp>;

    async fn find_all_transactiions_by_api_key(
        &self,
        request: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp>;
    async fn find_all_transactiions_by_id(
        &self,
        request: &FindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp>;
}
