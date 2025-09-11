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

pub type DynTopupStatsAmountByCardService =
    Arc<dyn TopupStatsAmountByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountByCardServiceTrait {
    async fn get_monthly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, ServiceError>;

    async fn get_yearly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, ServiceError>;
}
