use crate::{
    domain::{
        requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
        responses::{
            ApiResponse, ApiResponsePagination, TransactionResponse, TransactionResponseDeleteAt,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, AppErrorHttp>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, AppErrorHttp>;
    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp>;
    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, AppErrorHttp>;
}
