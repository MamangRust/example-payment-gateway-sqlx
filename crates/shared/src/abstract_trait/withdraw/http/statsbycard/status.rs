use crate::{
    domain::{
        requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
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

pub type DynWithdrawStatsStatusByCardNumberGrpcClient =
    Arc<dyn WithdrawStatsStatusByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusByCardNumberGrpcClientTrait {
    async fn find_month_status_success_by_card_number(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, AppErrorHttp>;
    async fn find_yearly_status_success_by_card_number(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn find_month_status_failed_by_card_number(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn find_yearly_status_failed_by_card_number(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, AppErrorHttp>;
}
