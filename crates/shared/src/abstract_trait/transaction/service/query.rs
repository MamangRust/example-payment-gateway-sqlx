use crate::{
    domain::{
        requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
        responses::{
            ApiResponse, ApiResponsePagination, TransactionResponse, TransactionResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionQueryService = Arc<dyn TransactionQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionQueryServiceTrait {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError>;
    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError>;
    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError>;
    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError>;
    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, ServiceError>;
}
