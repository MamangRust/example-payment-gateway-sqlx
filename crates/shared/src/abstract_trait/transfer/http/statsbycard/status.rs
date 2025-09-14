use crate::{
    domain::{
        requests::transfer::{MonthStatusTransferCardNumber, YearStatusTransferCardNumber},
        responses::{
            ApiResponse, TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
            TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransferStatsStatusByCardNumberGrpcClientTrait {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, AppErrorHttp>;

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, AppErrorHttp>;

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, AppErrorHttp>;
}
