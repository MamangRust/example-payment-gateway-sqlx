use crate::{
    domain::responses::{MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount},
    errors::RepositoryError,
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
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyTotalAmount>, RepositoryError>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyTotalAmount>, RepositoryError>;
}
