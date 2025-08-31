use crate::{
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountRepository =
    Arc<dyn MerchantStatsTotalAmountRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountRepositoryTrait {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyTotalAmount>, RepositoryError>;
}
