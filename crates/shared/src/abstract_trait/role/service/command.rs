use crate::{
    domain::{
        requests::role::{CreateRoleRequest, UpdateRoleRequest},
        responses::{ApiResponse, RoleResponse, RoleResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleCommandService = Arc<dyn RoleCommandServiceTrait + Send + Sync>;

#[async_trait]
pub trait RoleCommandServiceTrait {
    async fn create(
        &self,
        request: &CreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError>;
    async fn update(
        &self,
        request: &UpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError>;
}
