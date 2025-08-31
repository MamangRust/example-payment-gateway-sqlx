use crate::{
    domain::requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
    errors::RepositoryError,
    model::topup::TopupModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupQueryRepository = Arc<dyn TopupQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupQueryRepositoryTrait {
    async fn find_all(
        &self,
        request: FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError>;
    async fn find_all_by_card_number(
        &self,
        request: FindAllTopupsByCardNumber,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError>;
    async fn find_active(
        &self,
        request: FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError>;
    async fn find_trashed(
        &self,
        request: FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: String) -> Result<TopupModel, RepositoryError>;
}
