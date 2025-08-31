use crate::{
    domain::responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceService = Arc<dyn CardStatsBalanceServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceServiceTrait {
    async fn get_monthly_balance(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError>;
    async fn get_yearly_balance(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError>;
}
