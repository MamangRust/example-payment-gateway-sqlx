use crate::{
    domain::{
        requests::saldo::FindAllSaldos,
        responses::{ApiResponse, ApiResponsePagination, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoQueryGrpcClient = Arc<dyn SaldoQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait SaldoQueryGrpcClientTrait {
    async fn find_all(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, AppErrorHttp>;
    async fn find_active(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, AppErrorHttp>;
    async fn find_trashed(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, AppErrorHttp>;
}
