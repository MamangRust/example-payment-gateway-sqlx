use crate::{
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountByCardNumberGrpcClient =
    Arc<dyn TransferStatsAmountByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn get_monthly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;
}
