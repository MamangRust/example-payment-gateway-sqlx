use crate::{
    domain::{
        requests::saldo::FindAllSaldos,
        responses::{ApiResponse, ApiResponsePagination, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoQueryService = Arc<dyn SaldoQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait SaldoQueryServiceTrait {
    async fn find_all(
        &self,
        request: FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, ServiceError>;
    async fn find_active(
        &self,
        request: FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, ServiceError>;
    async fn find_trashed(
        &self,
        request: FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, ServiceError>;
    async fn find_by_id(&self, id: String) -> Result<ApiResponse<SaldoResponse>, ServiceError>;
}
