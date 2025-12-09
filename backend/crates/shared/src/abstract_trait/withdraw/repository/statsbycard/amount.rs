use crate::{
    domain::requests::withdraw::YearMonthCardNumber,
    errors::RepositoryError,
    model::withdraw::{WithdrawMonthlyAmount, WithdrawYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountByCardRepository =
    Arc<dyn WithdrawStatsAmountByCardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountByCardRepositoryTrait {
    async fn get_monthly_by_card(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<Vec<WithdrawMonthlyAmount>, RepositoryError>;

    async fn get_yearly_by_card(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<Vec<WithdrawYearlyAmount>, RepositoryError>;
}
