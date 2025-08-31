use crate::{
    domain::responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodGrpcClient = Arc<dyn TopupStatsMethodGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodGrpcClientTrait {
    async fn get_monthly_topup_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp>;

    async fn get_yearly_topup_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp>;
}
