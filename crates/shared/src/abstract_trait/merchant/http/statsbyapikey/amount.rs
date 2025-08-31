use crate::{
    domain::{
        requests::merchant::MonthYearAmountApiKey,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountByApiKeyService =
    Arc<dyn MerchantStatsAmountByApiKeyServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountByApiKeyServiceTrait {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp>;
    async fn get_yearly_amount(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp>;
}
