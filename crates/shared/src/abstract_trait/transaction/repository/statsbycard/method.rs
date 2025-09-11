use crate::{
    domain::requests::transaction::MonthYearPaymentMethod,
    errors::RepositoryError,
    model::transaction::{TransactionMonthMethod, TransactionYearMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsMethodByCardRepository =
    Arc<dyn TransactionStatsMethodByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsMethodByCardRepositoryTrait {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionMonthMethod>, RepositoryError>;
    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionYearMethod>, RepositoryError>;
}
