use crate::{
    domain::responses::{
        ApiResponse, MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantStatsMethodGrpcClientTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp>;
}
