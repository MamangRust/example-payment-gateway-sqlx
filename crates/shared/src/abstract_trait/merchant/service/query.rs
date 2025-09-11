use crate::{
    domain::{
        requests::merchant::FindAllMerchants,
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantQueryService = Arc<dyn MerchantQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantQueryServiceTrait {
    async fn find_all(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, ServiceError>;
    async fn find_active(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError>;
    async fn find_trashed(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, ServiceError>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError>;
}
