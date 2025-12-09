use crate::{
    domain::responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    errors::AppErrorHttp,
};
use async_trait::async_trait;

#[async_trait]
pub trait TopupStatsAmountGrpcClientTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp>;
}
