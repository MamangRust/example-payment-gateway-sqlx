use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTopupByCardGrpcClient =
    Arc<dyn CardStatsTopupByCardGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTopupByCardGrpcClientTrait {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<CardResponseMonthAmount>, AppErrorHttp>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<CardResponseYearAmount>, AppErrorHttp>;
}
