use crate::{
    domain::requests::merchant::FindAllMerchants, errors::RepositoryError,
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
        request: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError>;
    async fn find_active(
        &self,
        request: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError>;
    async fn find_trashed(
        &self,
        request: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<MerchantModel, RepositoryError>;
    async fn find_by_apikey(&self, api_key: String) -> Result<MerchantModel, RepositoryError>;
    async fn find_by_name(&self, name: String) -> Result<MerchantModel, RepositoryError>;
    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<MerchantModel>, RepositoryError>;
}
