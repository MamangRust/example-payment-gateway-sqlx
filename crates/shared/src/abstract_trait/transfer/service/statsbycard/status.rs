use crate::{
    domain::{
        requests::transfer::{MonthStatusTransferCardNumber, YearStatusTransferCardNumber},
        responses::{
            ApiResponse, TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
            TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsStatusByCardNumberService =
    Arc<dyn TransferStatsStatusByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusByCardNumberServiceTrait {
    async fn find_month_transfer_status_success_by_card_number(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError>;

    async fn find_yearly_transfer_status_success_by_card_number(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError>;

    async fn find_month_transfer_status_failed_by_card_number(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError>;

    async fn find_yearly_transfer_status_failed_by_card_number(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError>;
}
