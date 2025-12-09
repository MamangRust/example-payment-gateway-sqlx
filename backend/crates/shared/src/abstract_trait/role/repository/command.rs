use crate::{
    domain::requests::role::{CreateRoleRequest, UpdateRoleRequest},
    errors::RepositoryError,
    model::role::RoleModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleCommandRepository = Arc<dyn RoleCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait RoleCommandRepositoryTrait {
    async fn create(&self, request: &CreateRoleRequest) -> Result<RoleModel, RepositoryError>;
    async fn update(&self, request: &UpdateRoleRequest) -> Result<RoleModel, RepositoryError>;
    async fn trash(&self, id: i32) -> Result<RoleModel, RepositoryError>;
    async fn restore(&self, id: i32) -> Result<RoleModel, RepositoryError>;
    async fn delete_permanent(&self, id: i32) -> Result<(), RepositoryError>;
    async fn restore_all(&self) -> Result<(), RepositoryError>;
    async fn delete_all(&self) -> Result<(), RepositoryError>;
}
