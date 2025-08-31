use crate::{
    domain::requests::transaction::{
        MonthStatusTransactionCardNumber, YearStatusTransactionCardNumber,
    },
    errors::RepositoryError,
    model::transaction::{
        TransactionModelMonthStatusFailed, TransactionModelMonthStatusSuccess,
        TransactionModelYearStatusFailed, TransactionModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsStatusByCardNumberRepository =
    Arc<dyn TransactionStatsStatusByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusByCardNumberRepositoryTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<Vec<TransactionModelMonthStatusSuccess>, RepositoryError>;
    async fn get_yearly_status_success(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<Vec<TransactionModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<Vec<TransactionModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_status_failed(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<Vec<TransactionModelYearStatusFailed>, RepositoryError>;
}
