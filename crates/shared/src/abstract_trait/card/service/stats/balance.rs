use crate::{
    domain::responses::{CardResponseMonthBalance, CardResponseYearlyBalance},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsBalanceService = Arc<dyn CardStatsBalanceServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsBalanceServiceTrait {
    fn get_monthly_balance(&self) -> Result<Vec<CardResponseMonthBalance>, ServiceError>;
    fn get_yearly_balance(&self) -> Result<Vec<CardResponseYearlyBalance>, ServiceError>;
}
