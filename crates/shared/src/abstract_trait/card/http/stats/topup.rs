use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTopupGrpcClient = Arc<dyn CardStatsTopupGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTopupGrpcClientTrait {
    async fn get_monthly_amount(
        &self,
    ) -> Result<ApiResponse<CardResponseMonthAmount>, AppErrorHttp>;
    async fn get_yearly_amount(&self) -> Result<ApiResponse<CardResponseYearAmount>, AppErrorHttp>;
}
