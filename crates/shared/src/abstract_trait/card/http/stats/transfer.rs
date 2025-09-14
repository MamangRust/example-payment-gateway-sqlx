use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsTransferGrpcClientTrait {
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
}
