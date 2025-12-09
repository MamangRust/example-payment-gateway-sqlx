use crate::{
    errors::RepositoryError,
    model::transaction::{TransactionMonthAmount, TransactionYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountRepository =
    Arc<dyn TransactionStatsAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountRepositoryTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionMonthAmount>, RepositoryError>;

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionYearlyAmount>, RepositoryError>;
}
