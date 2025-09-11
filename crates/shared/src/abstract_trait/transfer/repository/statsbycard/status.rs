use crate::{
    domain::requests::transfer::{MonthStatusTransferCardNumber, YearStatusTransferCardNumber},
    errors::RepositoryError,
    model::transfer::{
        TransferModelMonthStatusFailed, TransferModelMonthStatusSuccess,
        TransferModelYearStatusFailed, TransferModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsStatusByCardRepository =
    Arc<dyn TransferStatsStatusByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusByCardRepositoryTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_status_success(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<Vec<TransferModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_status_failed(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<Vec<TransferModelYearStatusFailed>, RepositoryError>;
}
