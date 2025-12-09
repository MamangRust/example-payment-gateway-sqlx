use crate::{
    domain::requests::refresh_token::{CreateRefreshToken, UpdateRefreshToken},
    errors::RepositoryError,
    model::refresh_token::RefreshTokenModel,
};
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRefreshTokenCommandRepository =
    Arc<dyn RefreshTokenCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait RefreshTokenCommandRepositoryTrait {
    async fn create(
        &self,
        request: &CreateRefreshToken,
    ) -> Result<RefreshTokenModel, RepositoryError>;
    async fn update(
        &self,
        request: &UpdateRefreshToken,
    ) -> Result<RefreshTokenModel, RepositoryError>;
    async fn delete_token(&self, token: String) -> Result<(), RepositoryError>;
    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError>;
}
