use crate::{
    domain::{
        requests::merchant::MonthYearTotalAmountMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountByMerchantService =
    Arc<dyn MerchantStatsTotalAmountByMerchantServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountByMerchantServiceTrait {
    async fn find_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError>;
    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError>;
}
