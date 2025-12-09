use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransactionService = Arc<dyn CardStatsTransactionServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransactionServiceTrait {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
}
