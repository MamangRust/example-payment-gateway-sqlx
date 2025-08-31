use crate::{
    domain::{
        requests::role::FindAllRoles,
        responses::{ApiResponse, RoleResponse},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleQueryGrpcClient = Arc<dyn RoleQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait RoleQueryGrpcClientTrait {
    async fn find_all(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, AppErrorHttp>;
    async fn find_active(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, AppErrorHttp>;
    async fn find_trashed(
        &self,
        request: &FindAllRoles,
    ) -> Result<ApiResponse<Vec<RoleResponse>>, AppErrorHttp>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<RoleResponse>, AppErrorHttp>;
}
