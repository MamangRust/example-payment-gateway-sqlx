use crate::{
    domain::{
        requests::transaction::{
            MonthStatusTransactionCardNumber, YearStatusTransactionCardNumber,
        },
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

pub type DynTransactionStatsStatusByCardNumberService =
    Arc<dyn TransactionStatsStatusByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusByCardNumberServiceTrait {
    async fn find_month_status_success(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError>;
    async fn find_yearly_status_success(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError>;
    async fn find_month_status_failed(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError>;
    async fn find_yearly_status_failed(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError>;
}
