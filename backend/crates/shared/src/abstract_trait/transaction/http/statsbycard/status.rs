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

#[async_trait]
pub trait TransactionStatsStatusByCardNumberGrpcClientTrait {
    async fn get_month_status_success_bycard(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, AppErrorHttp>;
    async fn get_yearly_status_success_bycard(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_status_failed_bycard(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, AppErrorHttp>;
}
