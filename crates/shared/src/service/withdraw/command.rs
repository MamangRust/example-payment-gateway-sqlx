use crate::{
    domain::requests::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest, UpdateWithdrawStatus},
    domain::responses::{ApiResponse, WithdrawResponse, WithdrawResponseDeleteAt},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawCommandService = Arc<dyn WithdrawCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawCommandServiceTrait {
    async fn create(
        &self,
        req: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError>;
    async fn update(
        &self,
        req: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError>;
    async fn update_status(
        &self,
        req: &UpdateWithdrawStatus,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError>;
    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError>;
    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError>;
    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all_withdraw(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
