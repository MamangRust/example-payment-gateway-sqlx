use crate::{
    domain::requests::topup::MonthTopupStatus,
    errors::RepositoryError,
    model::topup::{
        TopupModelMonthStatusFailed, TopupModelMonthStatusSuccess, TopupModelYearStatusFailed,
        TopupModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsStatusRepository = Arc<dyn TopupStatsStatusRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusRepositoryTrait {
    async fn get_month_status_success(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<Vec<TopupModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<Vec<TopupModelYearStatusSuccess>, RepositoryError>;

    async fn get_month_status_failed(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<Vec<TopupModelMonthStatusFailed>, RepositoryError>;

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<Vec<TopupModelYearStatusFailed>, RepositoryError>;
}
