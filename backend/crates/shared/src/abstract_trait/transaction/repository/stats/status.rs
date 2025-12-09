use crate::{
    domain::requests::transaction::MonthStatusTransaction,
    errors::RepositoryError,
    model::transaction::{
        TransactionModelMonthStatusFailed, TransactionModelMonthStatusSuccess,
        TransactionModelYearStatusFailed, TransactionModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsStatusRepository =
    Arc<dyn TransactionStatsStatusRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusRepositoryTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<Vec<TransactionModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionModelYearStatusSuccess>, RepositoryError>;

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<Vec<TransactionModelMonthStatusFailed>, RepositoryError>;

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionModelYearStatusFailed>, RepositoryError>;
}
