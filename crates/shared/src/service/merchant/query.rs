use crate::{
    domain::{
        requests::merchant::FindAllMerchants,
        responses::{ApiResponse, MerchantResponse},
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
        request: FindAllMerchants,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError>;
    async fn find_active(
        &self,
        request: FindAllMerchants,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError>;
    async fn find_trashed(
        &self,
        request: FindAllMerchants,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, ServiceError>;
    async fn find_by_id(&self, id: String) -> Result<ApiResponse<MerchantResponse>, ServiceError>;
}
