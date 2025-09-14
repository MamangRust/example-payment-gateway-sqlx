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
pub trait CardStatsTransferByCardGrpcClientTrait {
    async fn get_monthly_amount_sender_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_sender_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
    async fn get_monthly_amount_receiver_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_receiver_bycard(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
}
