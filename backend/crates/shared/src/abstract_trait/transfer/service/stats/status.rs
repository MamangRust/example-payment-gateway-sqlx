use crate::{
    domain::{
        requests::transfer::MonthStatusTransfer,
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

pub type DynTransferStatsStatusService = Arc<dyn TransferStatsStatusServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsStatusServiceTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError>;

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError>;
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError>;
}
