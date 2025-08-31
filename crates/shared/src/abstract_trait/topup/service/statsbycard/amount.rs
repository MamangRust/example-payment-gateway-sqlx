use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountByCardNumberService =
    Arc<dyn TopupStatsAmountByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountByCardNumberServiceTrait {
    async fn get_monthly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError>;

    async fn get_yearly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError>;
}
