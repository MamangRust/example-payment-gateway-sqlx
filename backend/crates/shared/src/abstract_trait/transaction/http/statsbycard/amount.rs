use crate::{
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionStatsAmountByCardNumberGrpcClientTrait {
    async fn get_monthly_amounts_bycard(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp>;
    async fn get_yearly_amounts_bycard(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp>;
}
