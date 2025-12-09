use crate::{
    domain::{
        requests::card::{CreateCardRequest, UpdateCardRequest},
        responses::{ApiResponse, CardResponse, CardResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardCommandService = Arc<dyn CardCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait CardCommandServiceTrait {
    async fn create(
        &self,
        req: &CreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError>;
    async fn update(
        &self,
        req: &UpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
