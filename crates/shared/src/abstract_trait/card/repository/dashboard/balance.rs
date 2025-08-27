use crate::errors::RepositoryError;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardDashboardBalanceRepository =
    Arc<dyn CardDashboardBalanceRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardDashboardBalanceRepositoryTrait {
    async fn get_total_balance(&self) -> Result<i64, RepositoryError>;
    async fn get_total_balance_by_card(&self, card_number: String) -> Result<i64, RepositoryError>;
}
