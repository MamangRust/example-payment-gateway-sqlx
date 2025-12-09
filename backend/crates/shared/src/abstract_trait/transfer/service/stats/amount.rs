use crate::{
    domain::responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountService = Arc<dyn TransferStatsAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountServiceTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError>;
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError>;
}
