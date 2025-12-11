use crate::{
    domain::{
        requests::merchant::{CreateMerchantRequest, UpdateMerchantRequest},
        responses::{ApiResponse, MerchantResponse, MerchantResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantCommandGrpcClientTrait {
    async fn create(
        &self,
        request: &CreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, HttpError>;
    async fn update(
        &self,
        request: &UpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, HttpError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, HttpError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, HttpError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
