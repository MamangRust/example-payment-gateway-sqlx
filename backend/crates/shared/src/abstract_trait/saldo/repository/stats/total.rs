use crate::{
    domain::requests::saldo::MonthTotalSaldoBalance,
    errors::RepositoryError,
    model::saldo::{SaldoMonthTotalBalance, SaldoYearTotalBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoTotalBalanceRepository = Arc<dyn SaldoTotalBalanceRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait SaldoTotalBalanceRepositoryTrait {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<Vec<SaldoMonthTotalBalance>, RepositoryError>;
    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoYearTotalBalance>, RepositoryError>;
}
