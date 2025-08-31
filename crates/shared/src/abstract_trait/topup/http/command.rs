use crate::{
    domain::requests::topup::{CreateTopupRequest, UpdateTopupRequest},
    domain::responses::{ApiResponse, TopupResponse, TopupResponseDeleteAt},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupCommandGrpcClient = Arc<dyn TopupCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, AppErrorHttp>;
    async fn update(
        &self,
        req: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, AppErrorHttp>;
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, AppErrorHttp>;
    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, AppErrorHttp>;
    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn delete_all_permanent(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
