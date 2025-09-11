use crate::{
    domain::{
        requests::merchant::{CreateMerchantRequest, UpdateMerchantRequest},
        responses::{ApiResponse, MerchantResponse, MerchantResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantCommandGrpcClient = Arc<dyn MerchantCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantCommandGrpcClientTrait {
    async fn create(
        &self,
        request: &CreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp>;
    async fn update(
        &self,
        request: &UpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, AppErrorHttp>;
    async fn restore(&self, id: i32)
    -> Result<ApiResponse<MerchantResponseDeleteAt>, AppErrorHttp>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
