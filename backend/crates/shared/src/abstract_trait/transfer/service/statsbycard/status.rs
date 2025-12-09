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

pub type DynTransferStatsStatusByCardService =
    Arc<dyn TransferStatsStatusByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusByCardServiceTrait {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError>;

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError>;

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError>;

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError>;
}
