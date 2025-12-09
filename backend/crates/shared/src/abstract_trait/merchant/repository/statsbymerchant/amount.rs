use crate::{
    domain::requests::merchant::MonthYearAmountMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
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
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError>;
}
