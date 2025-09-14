use crate::{
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransferStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_amounts_sender_bycard(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn get_monthly_amounts_receiver_bycard(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts_sender_bycard(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;

    async fn get_yearly_amounts_receiver_bycard(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;
}
