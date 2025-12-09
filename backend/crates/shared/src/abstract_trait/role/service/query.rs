use crate::{
    domain::{
        requests::role::FindAllRoles,
        responses::{ApiResponse, ApiResponsePagination, RoleResponse, RoleResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleQueryService = Arc<dyn RoleQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait RoleQueryServiceTrait {
    async fn find_all(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponse>>, ServiceError>;
    async fn find_active(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError>;
    async fn find_trashed(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponsePagination<Vec<RoleResponseDeleteAt>>, ServiceError>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, ServiceError>;
    async fn find_by_user_id(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, ServiceError>;
    async fn find_by_name(&self, name: String) -> Result<ApiResponse<RoleResponse>, ServiceError>;
}
