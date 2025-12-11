use crate::{
    domain::{
        requests::saldo::FindAllSaldos,
        responses::{ApiResponse, ApiResponsePagination, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SaldoQueryGrpcClientTrait {
    async fn find_all(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, HttpError>;
    async fn find_active(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, HttpError>;
    async fn find_trashed(
        &self,
        request: &FindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, HttpError>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, HttpError>;
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError>;
}
