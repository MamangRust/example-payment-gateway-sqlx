use crate::{
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyPaymentMethod, MerchantYearlyPaymentMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodRepository =
    Arc<dyn MerchantStatsMethodRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodRepositoryTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyPaymentMethod>, RepositoryError>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyPaymentMethod>, RepositoryError>;
}
