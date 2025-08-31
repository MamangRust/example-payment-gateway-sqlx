use crate::{
    errors::RepositoryError,
    model::withdraw::{WithdrawMonthlyAmount, WithdrawYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawStatsAmountRepository =
    Arc<dyn WithdrawStatsAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawStatsAmountRepositoryTrait {
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawMonthlyAmount>, RepositoryError>;
    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawYearlyAmount>, RepositoryError>;
}
