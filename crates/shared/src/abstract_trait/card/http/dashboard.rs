use crate::{
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardDashboardGrpcClient = Arc<dyn CardDashboardGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardDashboardGrpcClientTrait {
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, AppErrorHttp>;
    async fn get_dashboard_bycard(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, AppErrorHttp>;
}
