use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsBalanceByCardGrpcClientTrait {
    async fn get_monthly_balance_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, AppErrorHttp>;
    async fn get_yearly_balance_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, AppErrorHttp>;
}
