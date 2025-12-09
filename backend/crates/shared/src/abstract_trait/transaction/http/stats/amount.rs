use crate::{
    domain::responses::{
        ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionStatsAmountGrpcClientTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp>;
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp>;
}
