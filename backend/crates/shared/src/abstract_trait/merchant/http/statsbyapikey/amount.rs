use crate::{
    domain::{
        requests::merchant::MonthYearAmountApiKey,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsAmountByApiKeyGrpcClientTrait {
    async fn get_monthly_amount_byapikey(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_byapikey(
        &self,
        req: &MonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp>;
}
