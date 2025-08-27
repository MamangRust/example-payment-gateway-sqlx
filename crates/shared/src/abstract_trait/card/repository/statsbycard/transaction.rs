use crate::{
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransactionByCardRepository =
    Arc<dyn CardStatsTransactionByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransactionByCardRepositoryTrait {
    async fn get_monthly_amount(&self, year: i32) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount(&self, year: i32) -> Result<Vec<CardYearAmount>, RepositoryError>;
}
