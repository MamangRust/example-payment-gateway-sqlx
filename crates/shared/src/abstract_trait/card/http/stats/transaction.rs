use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransactionGrpcClient =
    Arc<dyn CardStatsTransactionGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransactionGrpcClientTrait {
    async fn get_monthly_amount(
        &self,
    ) -> Result<ApiResponse<CardResponseMonthAmount>, AppErrorHttp>;
    async fn get_yearly_amount(&self) -> Result<ApiResponse<CardResponseYearAmount>, AppErrorHttp>;
}
