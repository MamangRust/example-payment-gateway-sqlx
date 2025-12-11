use crate::{
    domain::responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransferStatsAmountGrpcClientTrait {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, HttpError>;
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, HttpError>;
}
