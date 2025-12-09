use crate::{
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionStatsMethodByCardNumberGrpcClientTrait {
    async fn get_monthly_method_bycard(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, AppErrorHttp>;

    async fn get_yearly_method_bycard(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, AppErrorHttp>;
}
