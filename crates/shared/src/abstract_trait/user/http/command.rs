use crate::{
    domain::requests::user::{CreateUserRequest, UpdateUserRequest},
    domain::responses::{ApiResponse, UserResponse, UserResponseDeleteAt},
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserCommandGrpcClientTrait {
    async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;

    async fn update(
        &self,
        req: &UpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;

    async fn trashed(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, AppErrorHttp>;

    async fn restore(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, AppErrorHttp>;

    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;

    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;

    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
