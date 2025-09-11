use crate::{
    domain::requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
    errors::RepositoryError,
    model::withdraw::{
        WithdrawModelMonthStatusFailed, WithdrawModelMonthStatusSuccess,
        WithdrawModelYearStatusFailed, WithdrawModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsStatusByCardRepository =
    Arc<dyn WithdrawStatsStatusByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusByCardRepositoryTrait {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<Vec<WithdrawModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<Vec<WithdrawModelYearStatusSuccess>, RepositoryError>;

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
    ) -> Result<Vec<WithdrawModelMonthStatusFailed>, RepositoryError>;

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
    ) -> Result<Vec<WithdrawModelYearStatusFailed>, RepositoryError>;
}
