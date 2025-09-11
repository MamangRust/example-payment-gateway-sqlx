use crate::{
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountByCardService =
    Arc<dyn TransferStatsAmountByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardServiceTrait {
    async fn get_monthly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError>;

    async fn get_monthly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError>;

    async fn get_yearly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError>;

    async fn get_yearly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError>;
}
