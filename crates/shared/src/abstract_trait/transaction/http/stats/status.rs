use crate::{
    domain::{
        requests::transaction::MonthStatusTransaction,
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

pub type DynTransactionStatsStatusGrpcClient =
    Arc<dyn TransactionStatsStatusGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsStatusGrpcClientTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, AppErrorHttp>;
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, AppErrorHttp>;
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, AppErrorHttp>;
}
