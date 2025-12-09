use crate::{
    domain::responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TopupStatsMethodGrpcClientTrait {
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp>;

    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp>;
}
