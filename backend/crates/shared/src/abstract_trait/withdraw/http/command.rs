use crate::{
    domain::requests::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
    domain::responses::{ApiResponse, WithdrawResponse, WithdrawResponseDeleteAt},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WithdrawCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError>;
    async fn update(
        &self,
        req: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError>;
    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, HttpError>;
    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, HttpError>;
    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
