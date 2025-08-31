use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsWithdrawGrpcClient = Arc<dyn CardStatsWithdrawGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsWithdrawGrpcClientTrait {
    async fn get_monthly_amount(
        &self,
    ) -> Result<ApiResponse<CardResponseMonthAmount>, AppErrorHttp>;
    async fn get_yearly_amount(&self) -> Result<ApiResponse<CardResponseYearAmount>, AppErrorHttp>;
}
