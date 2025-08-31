use crate::{
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::{AppErrorHttp, ServiceError},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferStatsAmountByCardNumberGrpcClient =
    Arc<dyn TransferStatsAmountByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferStatsAmountByCardNumberGrpcClientTrait {
    async fn find_monthly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn find_monthly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp>;

    async fn find_yearly_transfer_amounts_by_sender_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;

    async fn find_yearly_transfer_amounts_by_receiver_card_number(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp>;
}
