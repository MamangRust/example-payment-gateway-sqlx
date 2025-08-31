use crate::{
    domain::responses::{
        MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodService = Arc<dyn MerchantStatsMethodServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodServiceTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyPaymentMethod>, ServiceError>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyPaymentMethod>, ServiceError>;
}
