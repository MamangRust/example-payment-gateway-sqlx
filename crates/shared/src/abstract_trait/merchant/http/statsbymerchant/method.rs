use crate::{
    domain::{
        requests::merchant::MonthYearPaymentMethodMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodByMerchantGrpcClient =
    Arc<dyn MerchantStatsMethodByMerchantGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodByMerchantGrpcClientTrait {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp>;
    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp>;
}
