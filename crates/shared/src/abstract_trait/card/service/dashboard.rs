use crate::{
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardDashboardService = Arc<dyn CardDashboardServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardDashboardServiceTrait {
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, ServiceError>;
    async fn get_dashboard_bycard(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, ServiceError>;
}
