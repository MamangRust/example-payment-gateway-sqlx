use crate::{
    domain::responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountService = Arc<dyn MerchantStatsAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountServiceTrait {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError>;
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError>;
}
