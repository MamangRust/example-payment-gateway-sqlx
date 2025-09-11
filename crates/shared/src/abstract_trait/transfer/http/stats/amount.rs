use crate::{
    domain::responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountGrpcClient =
    Arc<dyn TransferStatsAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountGrpcClientTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;
}
