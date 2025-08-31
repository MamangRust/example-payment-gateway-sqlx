use crate::{
    domain::{
        requests::merchant::MonthYearTotalAmountMerchant,
        responses::{
            ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountByMerchantService =
    Arc<dyn MerchantStatsTotalAmountByMerchantServiceTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountByMerchantServiceTrait {
    async fn get_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp>;
    async fn get_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp>;
}
