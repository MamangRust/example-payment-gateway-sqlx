use crate::{
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsAmountByCardNumberService =
    Arc<dyn TransactionStatsAmountByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsAmountByCardNumberServiceTrait {
    async fn find_monthly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError>;
    async fn find_yearly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError>;
}

