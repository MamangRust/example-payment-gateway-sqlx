use crate::{
    domain::{
        requests::withdraw::YearMonthCardNumber,
        responses::{ApiResponse, WithdrawMonthlyAmountResponse, WithdrawYearlyAmountResponse},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountByCardService =
    Arc<dyn WithdrawStatsAmountByCardServiceTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountByCardServiceTrait {
    async fn get_monthly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, ServiceError>;
    async fn get_yearly_by_card_number(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, ServiceError>;
}
