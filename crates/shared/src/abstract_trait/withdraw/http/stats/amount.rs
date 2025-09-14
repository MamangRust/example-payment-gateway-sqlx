use crate::{
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WithdrawStatsAmountGrpcClientTrait {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp>;
    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp>;
}
