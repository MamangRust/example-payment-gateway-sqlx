use crate::{
    domain::responses::{ApiResponse, MerchantResponseMonthlyAmount, MerchantResponseYearlyAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsAmountGrpcClient =
    Arc<dyn MerchantStatsAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsAmountGrpcClientTrait {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp>;
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp>;
}
