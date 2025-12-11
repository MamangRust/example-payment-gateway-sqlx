use crate::{
    domain::{
        requests::role::FindAllRoles,
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RoleQueryGrpcClientTrait {
    async fn find_all(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, HttpError>;
    async fn find_active(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, HttpError>;
    async fn find_trashed(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, HttpError>;
    async fn find_by_user_id(&self, id: i32) -> Result<ApiResponse<Vec<RoleResponse>>, HttpError>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, HttpError>;
}
