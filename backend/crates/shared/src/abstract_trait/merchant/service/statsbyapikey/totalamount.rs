use crate::{
    domain::{
        requests::merchant::MonthYearTotalAmountApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountByApiKeyService =
    Arc<dyn MerchantStatsTotalAmountByApiKeyServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountByApiKeyServiceTrait {
    async fn find_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, ServiceError>;

    async fn find_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, ServiceError>;
}
