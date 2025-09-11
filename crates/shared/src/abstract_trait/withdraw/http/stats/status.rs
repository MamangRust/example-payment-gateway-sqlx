use crate::{
    domain::{
        requests::withdraw::MonthStatusWithdraw,
        responses::{
            ApiResponse, WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
            WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsStatusGrpcClient =
    Arc<dyn WithdrawStatsStatusGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusGrpcClientTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, AppErrorHttp>;
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, AppErrorHttp>;
}
