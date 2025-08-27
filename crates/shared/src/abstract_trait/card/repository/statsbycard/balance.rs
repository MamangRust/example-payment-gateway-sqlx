use crate::{
    errors::RepositoryError,
    model::card::{CardMonthBalance, CardYearlyBalance},
};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceByCardRepository =
    Arc<dyn CardStatsBalanceByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceByCardRepositoryTrait {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardMonthBalance>, RepositoryError>;
    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardYearlyBalance>, RepositoryError>;
}
