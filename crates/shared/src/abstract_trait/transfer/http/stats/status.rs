use crate::{
    domain::{
        requests::transfer::MonthStatusTransfer,
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

pub type DynTransferStatsStatusGrpcClient =
    Arc<dyn TransferStatsStatusGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusGrpcClientTrait {
    async fn find_month_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn find_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn find_month_transfer_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn find_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, AppErrorHttp>;
}
