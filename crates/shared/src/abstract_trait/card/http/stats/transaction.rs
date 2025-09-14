use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsTransactionGrpcClientTrait {
    async fn get_monthly_transaction_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp>;
    async fn get_yearly_transaction_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp>;
}
