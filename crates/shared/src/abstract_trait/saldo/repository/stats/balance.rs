use crate::{
    errors::RepositoryError,
    model::saldo::{SaldoMonthSaldoBalance, SaldoYearSaldoBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoBalanceRepository = Arc<dyn SaldoBalanceRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait SaldoBalanceRepositoryTrait {
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoMonthSaldoBalance>, RepositoryError>;
    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoYearSaldoBalance>, RepositoryError>;
}
