use crate::{
    domain::requests::merchant::MonthYearPaymentMethodMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyPaymentMethod, MerchantYearlyPaymentMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByMerchantRepository =
    Arc<dyn MerchantStatsMethodByMerchantRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByMerchantRepositoryTrait {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<Vec<MerchantMonthlyPaymentMethod>, RepositoryError>;

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<Vec<MerchantYearlyPaymentMethod>, RepositoryError>;
}
