use crate::{
    domain::{
        requests::merchant::{CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus},
        responses::{ApiResponse, MerchantResponse, MerchantResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantCommandService = Arc<dyn MerchantCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantCommandServiceTrait {
    async fn create(
        &self,
        request: &CreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
    async fn update(
        &self,
        request: &UpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
    async fn update_status(
        &self,
        request: &UpdateMerchantStatus,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError>;
    async fn restore(&self, id: i32)
    -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
