use crate::{
    domain::requests::topup::{CreateTopupRequest, UpdateTopupRequest},
    domain::responses::{ApiResponse, TopupResponse, TopupResponseDeleteAt},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TopupCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, HttpError>;
    async fn update(
        &self,
        req: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, HttpError>;
    async fn trashed(&self, topup_id: i32)
    -> Result<ApiResponse<TopupResponseDeleteAt>, HttpError>;
    async fn restore(&self, topup_id: i32)
    -> Result<ApiResponse<TopupResponseDeleteAt>, HttpError>;
    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all_permanent(&self) -> Result<ApiResponse<bool>, HttpError>;
}
