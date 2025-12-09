use crate::{
    domain::{
        requests::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
        responses::{ApiResponse, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynSaldoCommandService = Arc<dyn SaldoCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait SaldoCommandServiceTrait {
    async fn create(
        &self,
        request: &CreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError>;
    async fn update(
        &self,
        request: &UpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
