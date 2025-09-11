use crate::{
    domain::{
        requests::card::{CreateCardRequest, UpdateCardRequest},
        responses::{ApiResponse, CardResponse, CardResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardCommandGrpcClient = Arc<dyn CardCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp>;
    async fn update(
        &self,
        req: &UpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, AppErrorHttp>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, AppErrorHttp>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
