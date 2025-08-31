use crate::{
    domain::{
        requests::saldo::MonthTotalSaldoBalance,
        responses::{ApiResponse, SaldoMonthTotalBalanceResponse, SaldoYearTotalBalanceResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoTotalBalanceGrpcClient = Arc<dyn SaldoTotalBalanceGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait SaldoTotalBalanceGrpcClientTrait {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, AppErrorHttp>;
    async fn get_year_total_balance(
        &self,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, AppErrorHttp>;
}
