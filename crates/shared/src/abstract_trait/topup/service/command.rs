use crate::{
    domain::requests::topup::{CreateTopupRequest, UpdateTopupRequest},
    domain::responses::{ApiResponse, TopupResponse, TopupResponseDeleteAt},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupCommandService = Arc<dyn TopupCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupCommandServiceTrait {
    async fn create(
        &self,
        req: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError>;
    async fn update(
        &self,
        req: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError>;
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError>;
    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError>;
    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
