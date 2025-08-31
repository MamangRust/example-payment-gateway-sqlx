use crate::{
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountByCardNumberGrpcClient =
    Arc<dyn TransactionStatsAmountByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountByCardNumberGrpcClientTrait {
    async fn find_monthly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp>;
    async fn find_yearly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp>;
}
