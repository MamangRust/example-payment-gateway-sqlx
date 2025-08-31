use crate::{
    domain::requests::transaction::{
        CreateTransactionRequest, UpdateTransactionRequest, UpdateTransactionStatus,
    },
    errors::RepositoryError,
    model::transaction::TransactionModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionCommandRepository = Arc<dyn TransactionCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionCommandRepositoryTrait {
    async fn create(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<TransactionModel, RepositoryError>;

    async fn update(
        &self,
        req: &UpdateTransactionRequest,
    ) -> Result<TransactionModel, RepositoryError>;

    async fn update_status(
        &self,
        req: &UpdateTransactionStatus,
    ) -> Result<TransactionModel, RepositoryError>;

    async fn trashed(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError>;

    async fn restore(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError>;

    async fn delete_permanent(&self, transaction_id: i32) -> Result<bool, RepositoryError>;

    async fn restore_all(&self) -> Result<bool, RepositoryError>;

    async fn delete_all_permanent(&self) -> Result<bool, RepositoryError>;
}
