use crate::{
    domain::responses::{ApiResponsePagination, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTopupByCardService = Arc<dyn CardStatsTopupByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTopupByCardServiceTrait {
    fn get_monthly_amount(
        &self,
    ) -> Result<ApiResponsePagination<Vec<CardResponseMonthAmount>>, ServiceError>;
    fn get_yearly_amount(
        &self,
    ) -> Result<ApiResponsePagination<Vec<CardResponseYearAmount>>, ServiceError>;
}
