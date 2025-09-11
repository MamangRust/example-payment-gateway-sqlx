use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceByCardGrpcClient =
    Arc<dyn CardStatsBalanceByCardGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceByCardGrpcClientTrait {
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, AppErrorHttp>;
    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, AppErrorHttp>;
}
