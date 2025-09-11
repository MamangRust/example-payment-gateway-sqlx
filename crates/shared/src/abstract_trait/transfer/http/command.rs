use crate::{
    domain::requests::transfer::{CreateTransferRequest, UpdateTransferRequest},
    domain::responses::{ApiResponse, TransferResponse, TransferResponseDeleteAt},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferCommandGrpcClient = Arc<dyn TransferCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp>;

    async fn update(
        &self,
        req: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp>;

    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, AppErrorHttp>;

    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, AppErrorHttp>;
    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;

    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
