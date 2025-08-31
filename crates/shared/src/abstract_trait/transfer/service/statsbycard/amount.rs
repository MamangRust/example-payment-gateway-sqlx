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

pub type DynTransferStatsAmountByCardNumberService =
    Arc<dyn TransferStatsAmountByCardNumberServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardNumberServiceTrait {
    async fn find_monthly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError>;

    async fn find_monthly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError>;

    async fn find_yearly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError>;

    async fn find_yearly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError>;
}
