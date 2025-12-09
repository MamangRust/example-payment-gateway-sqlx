use crate::{
    domain::{
        requests::merchant::MonthYearAmountMerchant,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountByMerchantService =
    Arc<dyn MerchantStatsAmountByMerchantServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountByMerchantServiceTrait {
    async fn find_monthly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError>;
    async fn find_yearly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError>;
}
