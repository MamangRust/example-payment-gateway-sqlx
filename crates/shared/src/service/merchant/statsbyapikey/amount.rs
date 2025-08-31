use crate::{
    domain::{
        requests::merchant::MonthYearAmountApiKey,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountByApiKeyService =
    Arc<dyn MerchantStatsAmountByApiKeyServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountByApiKeyServiceTrait {
    async fn find_monthly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, ServiceError>;

    async fn find_yearly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, ServiceError>;
}
