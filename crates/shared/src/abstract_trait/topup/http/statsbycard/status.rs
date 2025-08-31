use crate::{
    domain::{
        requests::topup::{MonthTopupStatusCardNumber, YearTopupStatusCardNumber},
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsStatusByCardNumberGrpcClient =
    Arc<dyn TopupStatsStatusByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusByCardNumberGrpcClientTrait {
    async fn get_month_topup_status_success(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn get_yearly_topup_status_success(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_topup_status_failed(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn get_yearly_topup_status_failed(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, AppErrorHttp>;
}
