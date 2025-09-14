use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TopupStatsMethodByCardNumberGrpcClientTrait {
    async fn get_monthly_methods_bycard(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp>;

    async fn get_yearly_methods_bycard(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp>;
}
