use crate::{
    domain::requests::topup::{MonthTopupStatusCardNumber, YearTopupStatusCardNumber},
    errors::RepositoryError,
    model::topup::{
        TopupModelMonthStatusFailed, TopupModelMonthStatusSuccess, TopupModelYearStatusFailed,
        TopupModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsStatusByCardNumberRepository =
    Arc<dyn TopupStatsStatusByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsStatusByCardNumberRepositoryTrait {
    async fn get_month_topup_status_success(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<Vec<TopupModelMonthStatusSuccess>, RepositoryError>;

    async fn get_yearly_topup_status_success(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<Vec<TopupModelYearStatusSuccess>, RepositoryError>;
    async fn get_month_topup_status_failed(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<Vec<TopupModelMonthStatusFailed>, RepositoryError>;
    async fn get_yearly_topup_status_failed(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<Vec<TopupModelYearStatusFailed>, RepositoryError>;
}
