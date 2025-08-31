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

pub type DynWithdrawStatsStatusByCardNumberService =
    Arc<dyn WithdrawStatsStatusByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusByCardNumberServiceTrait {
    async fn find_month_status_success_by_card_number(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, ServiceError>;
    async fn find_yearly_status_success_by_card_number(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, ServiceError>;
    async fn find_month_status_failed_by_card_number(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, ServiceError>;
    async fn find_yearly_status_failed_by_card_number(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, ServiceError>;
}
