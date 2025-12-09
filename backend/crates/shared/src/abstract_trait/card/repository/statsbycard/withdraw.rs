use crate::{
    domain::requests::card::MonthYearCardNumberCard,
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsWithdrawByCardRepository =
    Arc<dyn CardStatsWithdrawByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsWithdrawByCardRepositoryTrait {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardYearAmount>, RepositoryError>;
}
