use crate::{
    domain::responses::{MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyTotalAmount},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsTotalAmountGrpcClient =
    Arc<dyn MerchantStatsTotalAmountGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsTotalAmountGrpcClientTrait {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyTotalAmount>, AppErrorHttp>;
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyTotalAmount>, AppErrorHttp>;
}
