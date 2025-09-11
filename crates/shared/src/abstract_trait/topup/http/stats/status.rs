use crate::{
    domain::{
        requests::topup::MonthTopupStatus,
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsStatusGrpcClient = Arc<dyn TopupStatsStatusGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusGrpcClientTrait {
    async fn get_month_topup_status_success(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, AppErrorHttp>;

    async fn get_yearly_topup_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, AppErrorHttp>;
    async fn get_month_topup_status_failed(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, AppErrorHttp>;

    async fn get_yearly_topup_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, AppErrorHttp>;
}
