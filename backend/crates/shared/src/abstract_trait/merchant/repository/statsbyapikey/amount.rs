use crate::{
    domain::requests::merchant::MonthYearAmountApiKey,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountByApiKeyRepository =
    Arc<dyn MerchantStatsAmountByApiKeyRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountByApiKeyRepositoryTrait {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError>;
}
