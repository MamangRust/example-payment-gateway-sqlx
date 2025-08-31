use crate::{
    domain::requests::merchant::MonthYearTotalAmountMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountByMerchantRepository =
    Arc<dyn MerchantStatsTotalAmountByMerchantRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountByMerchantRepositoryTrait {
    async fn get_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError>;

    async fn get_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<Vec<MerchantYearlyTotalAmount>, RepositoryError>;
}
