use crate::{
    domain::{
        requests::card::{CreateCardRequest, UpdateCardRequest},
        responses::{ApiResponse, CardResponse, CardResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CardCommandGrpcClientTrait {
    async fn create(&self, req: &CreateCardRequest)
    -> Result<ApiResponse<CardResponse>, HttpError>;
    async fn update(&self, req: &UpdateCardRequest)
    -> Result<ApiResponse<CardResponse>, HttpError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, HttpError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, HttpError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
