use crate::{
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthAmountResponse, TopupYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountByCardNumberGrpcClient =
    Arc<dyn TopupStatsAmountByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp>;
}
