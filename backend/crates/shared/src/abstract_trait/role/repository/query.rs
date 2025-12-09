use crate::{
    domain::requests::role::FindAllRoles, errors::RepositoryError, model::role::RoleModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRoleQueryRepository = Arc<dyn RoleQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait RoleQueryRepositoryTrait {
    async fn find_all(&self, req: &FindAllRoles) -> Result<(Vec<RoleModel>, i64), RepositoryError>;
    async fn find_active(
        &self,
        req: &FindAllRoles,
    ) -> Result<(Vec<RoleModel>, i64), RepositoryError>;
    async fn find_trashed(
        &self,
        req: &FindAllRoles,
    ) -> Result<(Vec<RoleModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleModel>, RepositoryError>;
    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<RoleModel>, RepositoryError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<RoleModel>, RepositoryError>;
}
