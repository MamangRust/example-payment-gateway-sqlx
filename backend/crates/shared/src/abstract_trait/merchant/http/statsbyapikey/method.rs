use crate::{
    domain::{
        requests::merchant::MonthYearPaymentMethodApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsMethodByApiKeyGrpcClientTrait {
    async fn get_monthly_method_byapikey(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, HttpError>;
    async fn get_yearly_method_byapikey(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, HttpError>;
}
