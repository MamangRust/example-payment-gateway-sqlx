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
use std::sync::Arc;

pub type DynTransferStatsStatusByCardNumberGrpcClient =
    Arc<dyn TransferStatsStatusByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusByCardNumberGrpcClientTrait {
    async fn find_month_transfer_status_success_by_card_number(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn find_yearly_transfer_status_success_by_card_number(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, AppErrorHttp>;

    async fn find_month_transfer_status_failed_by_card_number(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, AppErrorHttp>;

    async fn find_yearly_transfer_status_failed_by_card_number(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, AppErrorHttp>;
}
