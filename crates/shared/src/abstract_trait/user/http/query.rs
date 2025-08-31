use crate::{
    domain::{
        requests::user::FindAllUserRequest,
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserQueryGrpcClient = Arc<dyn UserQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait UserQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, AppErrorHttp>;

    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;

    async fn find_by_active(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, AppErrorHttp>;

    async fn find_by_trashed(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, AppErrorHttp>;
}
