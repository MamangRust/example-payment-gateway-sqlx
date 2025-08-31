use crate::{
    errors::RepositoryError,
    model::transfer::{TransferMonthAmount, TransferYearAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountRepository =
    Arc<dyn TransferStatsAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountRepositoryTrait {
    async fn get_monthly_transfer_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransferMonthAmount>, RepositoryError>;

    async fn get_yearly_transfer_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransferYearAmount>, RepositoryError>;
}
