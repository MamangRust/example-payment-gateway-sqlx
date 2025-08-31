use crate::{
    domain::requests::transfer::MonthStatusTransfer,
    errors::RepositoryError,
    model::transfer::{
        TransferModelMonthStatusFailed, TransferModelMonthStatusSuccess,
        TransferModelYearStatusFailed, TransferModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsStatusRepository =
    Arc<dyn TransferStatsStatusRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusRepositoryTrait {
    async fn get_month_transfer_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<Vec<TransferModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_transfer_status_success(
        &self,
        year: i32,
    ) -> Result<Vec<TransferModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_transfer_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<Vec<TransferModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_transfer_status_failed(
        &self,
        year: i32,
    ) -> Result<Vec<TransferModelYearStatusFailed>, RepositoryError>;
}
