use crate::{
    domain::requests::transfer::{
        CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest,
        UpdateTransferStatus,
    },
    errors::RepositoryError,
    model::transfer::TransferModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferCommandRepository = Arc<dyn TransferCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferCommandRepositoryTrait {
    async fn create(&self, req: &CreateTransferRequest) -> Result<TransferModel, RepositoryError>;
    async fn update(&self, req: &UpdateTransferRequest) -> Result<TransferModel, RepositoryError>;
    async fn update_amount(
        &self,
        req: &UpdateTransferAmountRequest,
    ) -> Result<TransferModel, RepositoryError>;
    async fn update_status(
        &self,
        req: &UpdateTransferStatus,
    ) -> Result<TransferModel, RepositoryError>;
    async fn trashed(&self, transfer_id: i32) -> Result<TransferModel, RepositoryError>;
    async fn restore(&self, transfer_id: i32) -> Result<TransferModel, RepositoryError>;
    async fn delete_permanent(&self, transfer_id: i32) -> Result<bool, RepositoryError>;
    async fn restore_all(&self) -> Result<bool, RepositoryError>;
    async fn delete_all(&self) -> Result<bool, RepositoryError>;
}
