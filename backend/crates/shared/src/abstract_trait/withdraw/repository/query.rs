use crate::{
    domain::requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
    errors::RepositoryError,
    model::withdraw::WithdrawModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawQueryRepository = Arc<dyn WithdrawQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawQueryRepositoryTrait {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError>;

    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError>;

    async fn find_by_trashed(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError>;

    async fn find_all_by_card_number(
        &self,
        req: &FindAllWithdrawCardNumber,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError>;

    async fn find_by_id(&self, id: i32) -> Result<WithdrawModel, RepositoryError>;

    async fn find_by_card(&self, card_number: &str) -> Result<Vec<WithdrawModel>, RepositoryError>;
}
