use crate::{
    domain::responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceGrpcClient = Arc<dyn CardStatsBalanceGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceGrpcClientTrait {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, AppErrorHttp>;
    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, AppErrorHttp>;
}
