use crate::{
    domain::{
        requests::role::{CreateRoleRequest, UpdateRoleRequest},
        responses::{ApiResponse, RoleResponse, RoleResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RoleCommandGrpcClientTrait {
    async fn create(
        &self,
        request: &CreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, HttpError>;
    async fn update(
        &self,
        request: &UpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, HttpError>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, HttpError>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, HttpError>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError>;
}
