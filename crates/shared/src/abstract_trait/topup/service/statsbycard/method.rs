use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodByCardNumberService =
    Arc<dyn TopupStatsMethodByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodByCardNumberServiceTrait {
    async fn get_monthly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError>;

    async fn get_yearly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError>;
}
