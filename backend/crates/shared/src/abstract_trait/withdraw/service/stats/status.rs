use crate::{
    domain::{
        requests::withdraw::MonthStatusWithdraw,
        responses::{
            ApiResponse, WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
            WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsStatusService = Arc<dyn WithdrawStatsStatusServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusServiceTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError>;
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError>;
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError>;
}
