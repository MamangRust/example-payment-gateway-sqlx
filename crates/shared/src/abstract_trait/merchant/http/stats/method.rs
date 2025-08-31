use crate::{
    domain::responses::{
        MerchantResponseMonthlyPaymentMethod, MerchantResponseYearlyPaymentMethod,
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynMerchantStatsMethodGrpcClient =
    Arc<dyn MerchantStatsMethodGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait MerchantStatsMethodGrpcClientTrait {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseMonthlyPaymentMethod>, AppErrorHttp>;
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantResponseYearlyPaymentMethod>, AppErrorHttp>;
}
