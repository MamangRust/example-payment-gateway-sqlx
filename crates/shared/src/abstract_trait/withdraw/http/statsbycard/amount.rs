use crate::{
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WithdrawStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_bycard(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp>;
    async fn get_yearly_bycard(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp>;
}
