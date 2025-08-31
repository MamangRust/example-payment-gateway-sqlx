use crate::{
    domain::requests::transfer::FindAllTransfers, errors::RepositoryError,
    model::transfer::TransferModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferQueryRepository = Arc<dyn TransferQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferQueryRepositoryTrait {
    async fn find_all(
        &self,
        req: FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError>;

    async fn find_by_active(
        &self,
        req: FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError>;
    async fn find_by_trashed(
        &self,
        req: FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<TransferModel, RepositoryError>;
    async fn find_by_transfer_from(
        &self,
        transfer_from: String,
    ) -> Result<Vec<TransferModel>, RepositoryError>;
    async fn find_by_transfer_to(
        &self,
        transfer_to: String,
    ) -> Result<Vec<TransferModel>, RepositoryError>;
}
