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
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsStatusByCardNumberGrpcClient =
    Arc<dyn TransactionStatsStatusByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusByCardNumberGrpcClientTrait {
    async fn find_month_status_success(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, AppErrorHttp>;
    async fn find_yearly_status_success(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn find_month_status_failed(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn find_yearly_status_failed(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, AppErrorHttp>;
}
