use crate::{
    domain::{
        requests::saldo::MonthTotalSaldoBalance,
        responses::{ApiResponse, SaldoMonthTotalBalanceResponse, SaldoYearTotalBalanceResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoTotalBalanceService = Arc<dyn SaldoTotalBalanceServiceTrait + Send + Sync>;

#[async_trait]
pub trait SaldoTotalBalanceServiceTrait {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, ServiceError>;
    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, ServiceError>;
}
