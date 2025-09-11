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

pub type DynTopupStatsMethodByCardService =
    Arc<dyn TopupStatsMethodByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodByCardServiceTrait {
    async fn get_monthly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError>;

    async fn get_yearly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError>;
}
