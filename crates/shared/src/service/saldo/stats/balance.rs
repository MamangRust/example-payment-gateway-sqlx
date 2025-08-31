use crate::{
    domain::responses::{ApiResponse, SaldoMonthBalanceResponse, SaldoYearBalanceResponse},
    errors::ServiceError,
};
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoBalanceService = Arc<dyn SaldoBalanceServiceTrait + Send + Sync>;

#[async_trait]
pub trait SaldoBalanceServiceTrait {
    async fn get_month_balance(
        &self,
    ) -> Result<ApiResponse<Vec<SaldoMonthBalanceResponse>>, ServiceError>;
    async fn get_year_balance(
        &self,
    ) -> Result<ApiResponse<Vec<SaldoYearBalanceResponse>>, ServiceError>;
}
