use crate::{
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthMethod, TopupYearlyMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodByCardRepository =
    Arc<dyn TopupStatsMethodByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodByCardRepositoryTrait {
    async fn get_monthly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthMethod>, RepositoryError>;

    async fn get_yearly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyMethod>, RepositoryError>;
}
