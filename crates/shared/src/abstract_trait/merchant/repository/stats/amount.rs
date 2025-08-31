use crate::{
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountRepository =
    Arc<dyn MerchantStatsAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountRepositoryTrait {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError>;
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError>;
}
