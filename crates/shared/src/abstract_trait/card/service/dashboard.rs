use crate::{
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CardDashboardServiceTrait {
    async fn get_dashboard(&self) -> Result<ApiResponse<Vec<DashboardCard>>, ServiceError>;
    async fn get_dashboard_bycard(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, ServiceError>;
}

pub type DynCardDashboardService = Arc<dyn CardDashboardServiceTrait + Send + Sync>;
