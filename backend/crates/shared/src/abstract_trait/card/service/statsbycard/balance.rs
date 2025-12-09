use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
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
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, ServiceError>;
    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, ServiceError>;
}
