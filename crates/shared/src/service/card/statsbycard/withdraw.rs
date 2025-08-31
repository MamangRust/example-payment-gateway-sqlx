use crate::{
    domain::{
        requests::card::MonthYearCardNumberCard,
        responses::{ApiResponsePagination, CardResponseMonthAmount, CardResponseYearAmount},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsWithdrawByCardService =
    Arc<dyn CardStatsWithdrawByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsWithdrawByCardServiceTrait {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponsePagination<Vec<CardResponseMonthAmount>>, ServiceError>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<ApiResponsePagination<Vec<CardResponseYearAmount>>, ServiceError>;
}
