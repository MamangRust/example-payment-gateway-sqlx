use crate::{
    domain::{
        requests::user::FindAllUserRequest,
        responses::{ApiResponse, ApiResponsePagination, UserResponse, UserResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponse>>, HttpError>;

    async fn find_by_id(&self, user_id: i32) -> Result<ApiResponse<UserResponse>, HttpError>;

    async fn find_by_active(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, HttpError>;

    async fn find_by_trashed(
        &self,
        req: &FindAllUserRequest,
    ) -> Result<ApiResponsePagination<Vec<UserResponseDeleteAt>>, HttpError>;
}
