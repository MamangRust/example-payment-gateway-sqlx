use crate::{
    errors::RepositoryError,
    model::topup::{TopupMonthMethod, TopupYearlyMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupStatsMethodRepository = Arc<dyn TopupStatsMethodRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait TopupStatsMethodRepositoryTrait {
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<Vec<TopupMonthMethod>, RepositoryError>;

    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<Vec<TopupYearlyMethod>, RepositoryError>;
}
