use crate::{
    domain::requests::user::{CreateUserRequest, UpdateUserRequest},
    domain::responses::{ApiResponse, UserResponse, UserResponseDeleteAt},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserCommandService = Arc<dyn UserCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserCommandServiceTrait {
    async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError>;

    async fn update(
        &self,
        req: &UpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError>;

    async fn trashed(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError>;

    async fn restore(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError>;

    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, ServiceError>;

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;

    async fn delete_all_permanent(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
