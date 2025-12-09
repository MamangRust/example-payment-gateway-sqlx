use crate::{
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsTotalAmountGrpcClientTrait {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp>;
}
