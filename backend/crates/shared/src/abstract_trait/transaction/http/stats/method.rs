use crate::{
    domain::responses::{
        ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionStatsMethodGrpcClientTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, AppErrorHttp>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, AppErrorHttp>;
}
