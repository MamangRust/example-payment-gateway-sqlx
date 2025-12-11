use crate::{
    domain::requests::transaction::{CreateTransactionRequest, UpdateTransactionRequest},
    domain::responses::{ApiResponse, TransactionResponse, TransactionResponseDeleteAt},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionCommandGrpcClientTrait {
    async fn create(
        &self,
        api_key: &str,
        req: &CreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, HttpError>;
    async fn update(
        &self,
        api_key: &str,
        req: &UpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, HttpError>;
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, HttpError>;
    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, HttpError>;
    async fn delete_permanent(&self, transaction_id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
