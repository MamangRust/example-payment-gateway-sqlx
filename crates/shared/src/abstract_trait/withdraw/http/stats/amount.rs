use crate::{
    domain::responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountGrpcClient =
    Arc<dyn WithdrawStatsAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountGrpcClientTrait {
    async fn find_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp>;
    async fn find_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp>;
}
