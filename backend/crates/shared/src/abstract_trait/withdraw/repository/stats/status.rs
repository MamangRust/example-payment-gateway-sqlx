use crate::{
    domain::requests::withdraw::MonthStatusWithdraw,
    errors::RepositoryError,
    model::withdraw::{
        WithdrawModelMonthStatusFailed, WithdrawModelMonthStatusSuccess,
        WithdrawModelYearStatusFailed, WithdrawModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsStatusRepository =
    Arc<dyn WithdrawStatsStatusRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsStatusRepositoryTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<Vec<WithdrawModelMonthStatusSuccess>, RepositoryError>;
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_status_failed(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<Vec<WithdrawModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawModelYearStatusFailed>, RepositoryError>;
}
