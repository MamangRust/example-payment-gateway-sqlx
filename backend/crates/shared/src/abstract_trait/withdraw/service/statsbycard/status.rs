use crate::{
    domain::{
        requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
        responses::{
            ApiResponse, WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
            WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsStatusByCardService =
    Arc<dyn WithdrawStatsStatusByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusByCardServiceTrait {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError>;
    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError>;
    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError>;
    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError>;
}
