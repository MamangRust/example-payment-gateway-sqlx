use crate::{
    domain::requests::transaction::MonthYearPaymentMethod,
    errors::RepositoryError,
    model::transaction::{TransactionMonthAmount, TransactionYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountByCardRepository =
    Arc<dyn TransactionStatsAmountByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountByCardRepositoryTrait {
    async fn get_monthly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionMonthAmount>, RepositoryError>;
    async fn get_yearly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionYearlyAmount>, RepositoryError>;
}
