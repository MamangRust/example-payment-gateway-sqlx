use crate::{
    domain::{
        requests::user::FindAllUserRequest,
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserQueryService = Arc<dyn UserQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserQueryServiceTrait {
    async fn find_all(
        &self,
        req: FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, ServiceError>;

    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, ServiceError>;

    async fn find_by_active(
        &self,
        req: FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError>;

    async fn find_by_trashed(
        &self,
        req: FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, ServiceError>;
}
