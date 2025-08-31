use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodByCardNumberGrpcClient =
    Arc<dyn TopupStatsMethodByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodByCardNumberGrpcClientTrait {
    async fn get_monthly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp>;

    async fn get_yearly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp>;
}
