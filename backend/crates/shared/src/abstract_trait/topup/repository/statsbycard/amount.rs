use crate::{
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthAmount, TopupYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountByCardRepository =
    Arc<dyn TopupStatsAmountByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountByCardRepositoryTrait {
    async fn get_monthly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthAmount>, RepositoryError>;

    async fn get_yearly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyAmount>, RepositoryError>;
}
