use crate::{
    domain::responses::{
        ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountGrpcClient =
    Arc<dyn TransactionStatsAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountGrpcClientTrait {
    async fn find_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp>;
    async fn find_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp>;
}
