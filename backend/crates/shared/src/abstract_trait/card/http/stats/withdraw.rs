use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsWithdrawGrpcClientTrait {
    async fn get_monthly_withdraw_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_withdraw_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
}
