use crate::{
    domain::{
        requests::role::{CreateRoleRequest, UpdateRoleRequest},
        responses::{ApiResponse, RoleResponse, RoleResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleCommandGrpcClient = Arc<dyn RoleCommandGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait RoleCommandGrpcClientTrait {
    async fn create(
        &self,
        request: &CreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, AppErrorHttp>;
    async fn update(
        &self,
        request: &UpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, AppErrorHttp>;
    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, AppErrorHttp>;
    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, AppErrorHttp>;
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp>;
}
