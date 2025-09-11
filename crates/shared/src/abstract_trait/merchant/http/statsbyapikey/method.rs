use crate::{
    domain::{
        requests::merchant::MonthYearPaymentMethodApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByApiKeyGrpcClient =
    Arc<dyn MerchantStatsMethodByApiKeyGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByApiKeyGrpcClientTrait {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp>;
    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp>;
}
