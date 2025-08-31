use crate::{
    domain::requests::topup::{
        CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest, UpdateTopupStatus,
    },
    errors::RepositoryError,
    model::topup::TopupModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupCommandRepository = Arc<dyn TopupCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupCommandRepositoryTrait {
    async fn create(&self, req: &CreateTopupRequest) -> Result<TopupModel, RepositoryError>;
    async fn update(&self, req: &UpdateTopupRequest) -> Result<TopupModel, RepositoryError>;
    async fn update_amount(&self, req: &UpdateTopupAmount) -> Result<TopupModel, RepositoryError>;
    async fn update_status(&self, req: &UpdateTopupStatus) -> Result<TopupModel, RepositoryError>;
    async fn trashed(&self, topup_id: i32) -> Result<TopupModel, RepositoryError>;
    async fn restore(&self, topup_id: i32) -> Result<TopupModel, RepositoryError>;
    async fn delete_permanent(&self, topup_id: i32) -> Result<bool, RepositoryError>;
    async fn restore_all(&self) -> Result<bool, RepositoryError>;
    async fn delete_all_permanent(&self) -> Result<bool, RepositoryError>;
}
