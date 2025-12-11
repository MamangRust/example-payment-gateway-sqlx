use crate::{
    domain::{
        requests::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
        responses::{ApiResponse, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SaldoCommandGrpcClientTrait {
    async fn create(
        &self,
        request: &CreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError>;
    async fn update(
        &self,
        request: &UpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, HttpError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, HttpError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, HttpError>;
    async fn delete_permanent(&self, id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
