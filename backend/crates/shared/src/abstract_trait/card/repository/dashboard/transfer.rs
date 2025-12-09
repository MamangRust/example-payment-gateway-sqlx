use crate::errors::RepositoryError;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardDashboardTransferRepository =
    Arc<dyn CardDashboardTransferRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardDashboardTransferRepositoryTrait {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError>;
    async fn get_total_amount_by_sender(&self, card_number: String)
    -> Result<i64, RepositoryError>;
    async fn get_total_amount_by_receiver(
        &self,
        card_number: String,
    ) -> Result<i64, RepositoryError>;
}
