use crate::{
    domain::responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodService = Arc<dyn TopupStatsMethodServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodServiceTrait {
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError>;

    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError>;
}
