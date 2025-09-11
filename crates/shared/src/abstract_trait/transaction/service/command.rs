use crate::{
    domain::{
        requests::transaction::{CreateTransactionRequest, UpdateTransactionRequest},
        responses::{ApiResponse, TransactionResponse, TransactionResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionCommandService = Arc<dyn TransactionCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionCommandServiceTrait {
    async fn create(
        &self,
        api_key: &str,
        req: &CreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError>;
    async fn update(
        &self,
        api_key: &str,
        req: &UpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError>;
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, ServiceError>;
    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, ServiceError>;
    async fn delete_permanent(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
