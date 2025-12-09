use crate::{
    domain::responses::{ApiResponse, DashboardCard, DashboardCardCardNumber},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardDashboardGrpcClientTrait {
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, AppErrorHttp>;
    async fn get_dashboard_bycard(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, AppErrorHttp>;
}
