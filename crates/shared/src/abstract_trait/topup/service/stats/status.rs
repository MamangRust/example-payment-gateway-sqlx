use crate::{
    domain::{
        requests::topup::MonthTopupStatus,
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use genproto::topup::TopupYearStatusSuccessResponse;
use std::sync::Arc;

pub type DynTopupStatsStatusService = Arc<dyn TopupStatsStatusServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusServiceTrait {
    async fn get_month_topup_status_success(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError>;

    async fn get_yearly_topup_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError>;
    async fn get_month_topup_status_failed(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError>;

    async fn get_yearly_topup_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError>;
}
