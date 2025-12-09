use crate::{
    domain::requests::transfer::{CreateTransferRequest, UpdateTransferRequest},
    domain::responses::{ApiResponse, TransferResponse, TransferResponseDeleteAt},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferCommandService = Arc<dyn TransferCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferCommandServiceTrait {
    async fn create(
        &self,
        req: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError>;

    async fn update(
        &self,
        req: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError>;

    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, ServiceError>;

    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, ServiceError>;
    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
