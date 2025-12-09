use crate::{
    errors::RepositoryError,
    model::topup::{TopupMonthAmount, TopupYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsAmountRepository = Arc<dyn TopupStatsAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsAmountRepositoryTrait {
    async fn get_monthly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TopupMonthAmount>, RepositoryError>;
    async fn get_yearly_topup_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TopupYearlyAmount>, RepositoryError>;
}
