use crate::{
    domain::requests::merchant::{
        CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus,
    },
    errors::RepositoryError,
    model::merchant::MerchantModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantCommandRepository = Arc<dyn MerchantCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantCommandRepositoryTrait {
    async fn create(
        &self,
        api_key: String,
        request: &CreateMerchantRequest,
    ) -> Result<MerchantModel, RepositoryError>;
    async fn update(
        &self,
        request: &UpdateMerchantRequest,
    ) -> Result<MerchantModel, RepositoryError>;
    async fn update_status(
        &self,
        request: UpdateMerchantStatus,
    ) -> Result<MerchantModel, RepositoryError>;
    async fn trash(&self, id: i32) -> Result<MerchantModel, RepositoryError>;
    async fn restore(&self, id: i32) -> Result<MerchantModel, RepositoryError>;
    async fn delete_permanent(&self, id: i32) -> Result<(), RepositoryError>;
    async fn restore_all(&self) -> Result<(), RepositoryError>;
    async fn delete_all(&self) -> Result<(), RepositoryError>;
}
