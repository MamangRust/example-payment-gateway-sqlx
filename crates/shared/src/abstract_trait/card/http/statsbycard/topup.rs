use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsTopupByCardGrpcClientTrait {
    async fn get_monthly_topup_amount_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_topup_amount_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
}
