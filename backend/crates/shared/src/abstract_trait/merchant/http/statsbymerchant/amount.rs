use crate::{
    domain::{
        requests::merchant::MonthYearAmountMerchant,
        responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsAmountByMerchantGrpcClientTrait {
    async fn get_monthly_amount_bymerchant(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp>;
    async fn get_yearly_amount_bymerchant(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp>;
}
