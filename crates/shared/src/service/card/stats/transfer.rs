use crate::{
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardStatsTransferService = Arc<dyn CardStatsTransferServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardStatsTransferServiceTrait {
    fn get_monthly_amount_sender(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    fn get_yearly_amount_sender(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
    fn get_monthly_amount_receiver(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    fn get_yearly_amount_receiver(
        &self,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
}
