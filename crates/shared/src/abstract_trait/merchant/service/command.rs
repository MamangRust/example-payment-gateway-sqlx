use crate::{
    domain::requests::merchant::{
        CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus,
    },
    errors::ServiceError,
    model::merchant::MerchantModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantCommandService = Arc<dyn MerchantCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantCommandServiceTrait {
    async fn create(
        &self,
        api_key: String,
        request: &CreateMerchantRequest,
    ) -> Result<MerchantModel, ServiceError>;
    async fn update(&self, request: UpdateMerchantRequest) -> Result<MerchantModel, ServiceError>;
    async fn update_status(
        &self,
        request: &UpdateMerchantStatus,
    ) -> Result<MerchantModel, ServiceError>;
    async fn trash(&self, id: i32) -> Result<MerchantModel, ServiceError>;
    async fn restore(&self, id: i32) -> Result<MerchantModel, ServiceError>;
    async fn delete(&self, id: i32) -> Result<MerchantModel, ServiceError>;
    async fn restore_all(&self) -> Result<MerchantModel, ServiceError>;
    async fn delete_all(&self) -> Result<MerchantModel, ServiceError>;
}
