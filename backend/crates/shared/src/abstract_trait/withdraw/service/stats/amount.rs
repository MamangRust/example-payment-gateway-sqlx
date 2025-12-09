use crate::{
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountService = Arc<dyn WithdrawStatsAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountServiceTrait {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError>;
    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError>;
}
