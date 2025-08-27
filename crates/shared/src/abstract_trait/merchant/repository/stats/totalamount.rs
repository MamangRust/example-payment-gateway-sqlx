use crate::{
    domain::responses::{MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount},
    errors::RepositoryError,
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
    ) -> Result<Vec<MerchantResponseMonthlyTotalAmount>, RepositoryError>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyTotalAmount>, RepositoryError>;
}
