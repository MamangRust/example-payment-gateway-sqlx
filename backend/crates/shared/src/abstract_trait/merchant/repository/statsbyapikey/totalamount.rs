use crate::{
    domain::requests::merchant::MonthYearTotalAmountApiKey,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountByApiKeyRepository =
    Arc<dyn MerchantStatsTotalAmountByApiKeyRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountByApiKeyRepositoryTrait {
    async fn get_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError>;
    async fn get_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<Vec<MerchantYearlyTotalAmount>, RepositoryError>;
}
