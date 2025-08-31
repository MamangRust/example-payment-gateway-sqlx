use crate::{
    domain::{
        requests::transaction::MonthStatusTransaction,
        responses::{
            ApiResponse, TransactionResponseMonthStatusFailed,
            TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
            TransactionResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsStatusService =
    Arc<dyn TransactionStatsStatusServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusServiceTrait {
    async fn find_month_status_success(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError>;
    async fn find_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError>;
    async fn find_month_status_failed(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError>;
    async fn find_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError>;
}
