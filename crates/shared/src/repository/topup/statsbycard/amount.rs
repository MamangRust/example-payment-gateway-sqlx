use crate::{
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthAmount, TopupYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountByCardNumberRepository =
    Arc<dyn TopupStatsAmountByCardNumberRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountByCardNumberRepositoryTrait {
    async fn get_monthly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthAmount>, RepositoryError>;

    async fn get_yearly_topup_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyAmount>, RepositoryError>;
}
