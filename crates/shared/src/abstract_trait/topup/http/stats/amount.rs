use crate::{
    domain::responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    errors::AppErrorHttp,
};
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountGrpcClient = Arc<dyn TopupStatsAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountGrpcClientTrait {
    async fn get_monthly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp>;
}
