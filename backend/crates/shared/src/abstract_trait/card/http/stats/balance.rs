use crate::{
    domain::responses::{ApiResponse, CardResponseMonthBalance, CardResponseYearlyBalance},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardStatsBalanceGrpcClientTrait {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, HttpError>;
    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, HttpError>;
}
