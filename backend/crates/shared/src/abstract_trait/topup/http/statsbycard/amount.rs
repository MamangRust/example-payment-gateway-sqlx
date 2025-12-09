use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TopupStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_amounts_bycard(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts_bycard(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp>;
}
