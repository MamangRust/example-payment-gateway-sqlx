use crate::{
    domain::requests::transfer::MonthYearCardNumber,
    errors::RepositoryError,
    model::transfer::{
        TransferModelMonthStatusFailed, TransferModelMonthStatusSuccess,
        TransferModelYearStatusFailed, TransferModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsStatusByCardNumberRepository =
    Arc<dyn TransferStatsStatusByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusByCardNumberRepositoryTrait {
    async fn get_month_transfer_status_success_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusSuccess>, RepositoryError>;
    async fn get_yearly_transfer_status_success_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_transfer_status_failed_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_transfer_status_failed_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelYearStatusFailed>, RepositoryError>;
    async fn get_month_transfer_status_success_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusSuccess>, RepositoryError>;
    async fn get_yearly_transfer_status_success_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_transfer_status_failed_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_transfer_status_failed_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<Vec<TransferModelYearStatusFailed>, RepositoryError>;
}
