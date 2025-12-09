use crate::errors::RepositoryError;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardDashboardTopupRepository = Arc<dyn CardDashboardTopupRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardDashboardTopupRepositoryTrait {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError>;
    async fn get_total_amount_by_card(&self, card_number: String) -> Result<i64, RepositoryError>;
}
