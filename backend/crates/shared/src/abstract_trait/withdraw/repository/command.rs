use crate::{
    domain::requests::withdraw::{
        CreateWithdrawRequest, UpdateWithdrawRequest, UpdateWithdrawStatus,
    },
    errors::RepositoryError,
    model::withdraw::WithdrawModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawCommandRepository = Arc<dyn WithdrawCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawCommandRepositoryTrait {
    async fn create(&self, req: &CreateWithdrawRequest) -> Result<WithdrawModel, RepositoryError>;

    async fn update(&self, req: &UpdateWithdrawRequest) -> Result<WithdrawModel, RepositoryError>;

    async fn update_status(
        &self,
        req: &UpdateWithdrawStatus,
    ) -> Result<WithdrawModel, RepositoryError>;

    async fn trashed(&self, withdraw_id: i32) -> Result<WithdrawModel, RepositoryError>;
    async fn restore(&self, withdraw_id: i32) -> Result<WithdrawModel, RepositoryError>;
    async fn delete_permanent(&self, withdraw_id: i32) -> Result<bool, RepositoryError>;
    async fn restore_all(&self) -> Result<bool, RepositoryError>;
    async fn delete_all(&self) -> Result<bool, RepositoryError>;
}
