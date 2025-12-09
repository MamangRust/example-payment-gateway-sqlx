use crate::{
    domain::requests::merchant::MonthYearPaymentMethodApiKey,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyPaymentMethod, MerchantYearlyPaymentMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByApiKeyRepository =
    Arc<dyn MerchantStatsMethodByApiKeyRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByApiKeyRepositoryTrait {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<Vec<MerchantMonthlyPaymentMethod>, RepositoryError>;
    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<Vec<MerchantYearlyPaymentMethod>, RepositoryError>;
}
