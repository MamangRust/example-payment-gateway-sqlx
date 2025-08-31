use crate::{
    domain::{
        requests::merchant::MonthYearPaymentMethodApiKey,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByApiKeyService =
    Arc<dyn MerchantStatsMethodByApiKeyServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByApiKeyServiceTrait {
    async fn find_monthly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, ServiceError>;

    async fn find_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, ServiceError>;
}
