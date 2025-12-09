use crate::{
    domain::{
        requests::merchant::MonthYearTotalAmountApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsTotalAmountByApiKeyGrpcClientTrait {
    async fn get_monthly_total_amount_byapikey(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp>;
    async fn get_yearly_total_amount_byapikey(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp>;
}
