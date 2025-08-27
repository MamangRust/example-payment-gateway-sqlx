use crate::{
    domain::responses::{MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    errors::RepositoryError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountByMerchantRepository =
    Arc<dyn MerchantStatsAmountByMerchantRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountByMerchantRepositoryTrait {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyAmount>, RepositoryError>;
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyAmount>, RepositoryError>;
}
