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

#[async_trait]
pub trait TopupStatsStatusByCardNumberGrpcClientTrait {
    async fn get_month_status_success_bycard(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn get_yearly_status_success_bycard(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_status_failed_bycard(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, AppErrorHttp>;
}
