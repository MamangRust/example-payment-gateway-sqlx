use crate::{
    domain::{
        requests::topup::{MonthTopupStatusCardNumber, YearTopupStatusCardNumber},
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsStatusByCardNumberService =
    Arc<dyn TopupStatsStatusByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusByCardNumberServiceTrait {
    async fn get_month_topup_status_success(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError>;

    async fn get_yearly_topup_status_success(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError>;
    async fn get_month_topup_status_failed(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError>;
    async fn get_yearly_topup_status_failed(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError>;
}
