use crate::{
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransferRepository = Arc<dyn CardStatsTransferRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransferRepositoryTrait {
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<Vec<CardYearAmount>, RepositoryError>;
    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError>;
    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<Vec<CardYearAmount>, RepositoryError>;
}
