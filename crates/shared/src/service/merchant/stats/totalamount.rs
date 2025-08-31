use crate::{
    domain::responses::{MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountService =
    Arc<dyn MerchantStatsTotalAmountServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountServiceTrait {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyTotalAmount>, ServiceError>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyTotalAmount>, ServiceError>;
}
