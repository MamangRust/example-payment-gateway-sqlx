use crate::{
    domain::requests::transfer::{CreateTransferRequest, UpdateTransferRequest},
    domain::responses::{ApiResponse, TransferResponse, TransferResponseDeleteAt},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransferCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, HttpError>;

    async fn update(
        &self,
        req: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, HttpError>;

    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, HttpError>;

    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, HttpError>;
    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;

    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
