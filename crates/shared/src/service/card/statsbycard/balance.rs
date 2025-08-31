use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{CardResponseMonthBalance, CardResponseYearlyBalance},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceByCardService =
    Arc<dyn CardStatsBalanceByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceByCardServiceTrait {
    fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardResponseMonthBalance>, ServiceError>;
    fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardResponseYearlyBalance>, ServiceError>;
}
