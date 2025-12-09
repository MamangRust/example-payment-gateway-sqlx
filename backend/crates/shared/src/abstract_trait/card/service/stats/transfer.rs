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
    async fn get_monthly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    async fn get_yearly_amount_sender(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
    async fn get_monthly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError>;
    async fn get_yearly_amount_receiver(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError>;
}
