use crate::{
    domain::responses::{
        MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
    errors::RepositoryError,
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
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyPaymentMethod>, RepositoryError>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyPaymentMethod>, RepositoryError>;
}
