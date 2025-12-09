use crate::{
    domain::{
        requests::merchant::MonthYearPaymentMethodMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByMerchantService =
    Arc<dyn MerchantStatsMethodByMerchantServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByMerchantServiceTrait {
    async fn find_monthly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError>;

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError>;
}
