use crate::{
    domain::requests::transaction::{CreateTransactionRequest, UpdateTransactionRequest},
    domain::responses::{ApiResponse, TransactionResponse, TransactionResponseDeleteAt},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionCommandGrpcClient = Arc<dyn TransactionCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionCommandGrpcClientTrait {
    async fn create(
        &self,
        api_key: &str,
        req: &CreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp>;
    async fn update(
        &self,
        api_key: &str,
        req: &UpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp>;
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, AppErrorHttp>;
    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, AppErrorHttp>;
    async fn delete_permanent(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
