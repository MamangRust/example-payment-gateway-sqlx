use crate::{
    domain::responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    errors::ServiceError,
};
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountService = Arc<dyn TopupStatsAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountServiceTrait {
    async fn get_monthly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError>;

    async fn get_yearly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError>;
}
