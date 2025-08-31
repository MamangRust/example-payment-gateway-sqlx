use crate::{
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountByCardNumberGrpcClient =
    Arc<dyn WithdrawStatsAmountByCardNumberGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountByCardNumberGrpcClientTrait {
    async fn find_monthly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp>;
    async fn find_yearly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp>;
}
