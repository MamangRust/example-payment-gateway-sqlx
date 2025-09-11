use crate::{
    domain::responses::{
        ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransactionStatsMethodService =
    Arc<dyn TransactionStatsMethodServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransactionStatsMethodServiceTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError>;
}
