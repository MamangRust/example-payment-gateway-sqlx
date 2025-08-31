use crate::{
    domain::requests::user::FindAllUserRequest, errors::RepositoryError, model::user::UserModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserQueryRepository = Arc<dyn UserQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait UserQueryRepositoryTrait {
    async fn find_all(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError>;

    async fn find_by_active(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError>;

    async fn find_by_trashed(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError>;

    async fn find_by_id(&self, user_id: i32) -> Result<UserModel, RepositoryError>;

    async fn find_by_email(&self, email: String) -> Result<UserModel, RepositoryError>;
}
