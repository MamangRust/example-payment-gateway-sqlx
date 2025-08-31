use crate::{
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsMethodByCardNumberGrpcClient =
    Arc<dyn TransactionStatsMethodByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsMethodByCardNumberGrpcClientTrait {
    async fn find_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, AppErrorHttp>;

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, AppErrorHttp>;
}
