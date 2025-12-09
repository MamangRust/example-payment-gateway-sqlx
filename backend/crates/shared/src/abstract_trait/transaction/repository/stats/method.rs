use crate::{
    errors::RepositoryError,
    model::transaction::{TransactionMonthMethod, TransactionYearMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsMethodRepository =
    Arc<dyn TransactionStatsMethodRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsMethodRepositoryTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionMonthMethod>, RepositoryError>;

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionYearMethod>, RepositoryError>;
}
