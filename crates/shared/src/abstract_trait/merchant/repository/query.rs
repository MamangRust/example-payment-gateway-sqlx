use crate::{
    domain::{
        requests::FindAllMerchants,
        responses::{ApiResponse, ApiResponsePagination, MerchantResponse},
    },
    errors::{RepositoryError, ServiceError},
    model::merchant::MerchantModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantQueryRepository = Arc<dyn MerchantQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantQueryRepositoryTrait {
    async fn find_all(
        &self,
        request: FindAllMerchants,
    ) -> Result<Vec<MerchantModel>, RepositoryError>;
    async fn find_active(
        &self,
        request: FindAllMerchants,
    ) -> Result<Vec<MerchantModel>, RepositoryError>;
    async fn find_trashed(
        &self,
        request: FindAllMerchants,
    ) -> Result<Vec<MerchantModel>, RepositoryError>;
    async fn find_by_id(&self, id: String) -> Result<MerchantModel, RepositoryError>;
}
