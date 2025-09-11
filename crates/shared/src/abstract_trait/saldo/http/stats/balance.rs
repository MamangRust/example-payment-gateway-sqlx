use crate::{
    domain::responses::{ApiResponse, SaldoMonthBalanceResponse, SaldoYearBalanceResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoBalanceGrpcClient = Arc<dyn SaldoBalanceGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait SaldoBalanceGrpcClientTrait {
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoMonthBalanceResponse>>, AppErrorHttp>;
    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearBalanceResponse>>, AppErrorHttp>;
}
