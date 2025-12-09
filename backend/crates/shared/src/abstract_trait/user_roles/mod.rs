use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::requests::user_role::{CreateUserRoleRequest, RemoveUserRoleRequest},
    errors::RepositoryError,
    model::user_role::UserRoleModel,
};

pub type DynUserRoleCommandRepository = Arc<dyn UserRoleCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserRoleCommandRepositoryTrait {
    async fn assign_role_to_user(
        &self,
        req: &CreateUserRoleRequest,
    ) -> Result<UserRoleModel, RepositoryError>;
    async fn remove_role_from_user(
        &self,
        req: &RemoveUserRoleRequest,
    ) -> Result<(), RepositoryError>;
}
