use crate::{
    domain::requests::card::MonthYearCardNumberCard,
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransferByCardRepository =
    Arc<dyn CardStatsTransferByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransferByCardRepositoryTrait {
    async fn get_monthly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount_sender(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardYearAmount>, RepositoryError>;
    async fn get_monthly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount_receiver(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardYearAmount>, RepositoryError>;
}
