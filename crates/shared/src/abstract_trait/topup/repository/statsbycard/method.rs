use crate::{
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthMethod, TopupYearlyMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodByCardNumberRepository =
    Arc<dyn TopupStatsMethodByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodByCardNumberRepositoryTrait {
    async fn get_monthly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthMethod>, RepositoryError>;

    async fn get_yearly_topup_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyMethod>, RepositoryError>;
}
