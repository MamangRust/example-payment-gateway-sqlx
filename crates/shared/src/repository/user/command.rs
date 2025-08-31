use crate::{
    domain::requests::user::{CreateUserRequest, UpdateUserRequest},
    errors::RepositoryError,
    model::user::UserModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserCommandRepository = Arc<dyn UserCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserCommandRepositoryTrait {
    async fn create(&self, req: &CreateUserRequest) -> Result<UserModel, RepositoryError>;
    async fn update(&self, req: &UpdateUserRequest) -> Result<UserModel, RepositoryError>;
    async fn trashed(&self, user_id: i32) -> Result<UserModel, RepositoryError>;
    async fn restore(&self, user_id: i32) -> Result<UserModel, RepositoryError>;
    async fn delete_permanent(&self, user_id: i32) -> Result<bool, RepositoryError>;
    async fn restore_all(&self) -> Result<bool, RepositoryError>;
    async fn delete_all_permanent(&self) -> Result<bool, RepositoryError>;
}
