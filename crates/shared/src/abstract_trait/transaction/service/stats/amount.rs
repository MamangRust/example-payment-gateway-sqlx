use crate::{
    domain::responses::{
        ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountService =
    Arc<dyn TransactionStatsAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountServiceTrait {
    async fn find_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError>;
    async fn find_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError>;
}
