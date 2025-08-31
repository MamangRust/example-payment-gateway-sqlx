use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsWithdrawService = Arc<dyn CardStatsWithdrawServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsWithdrawServiceTrait {
    fn get_monthly_amount(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    fn get_yearly_amount(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
}
