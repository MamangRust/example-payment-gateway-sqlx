use crate::{errors::RepositoryError, model::refresh_token::RefreshTokenModel};
use async_trait::async_trait;
use std::sync::Arc;

pub type DynRefreshTokenQueryRepository = Arc<dyn RefreshTokenQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait RefreshTokenQueryRepositoryTrait {
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<Option<RefreshTokenModel>, RepositoryError>;
    async fn find_by_token(
        &self,
        token: String,
    ) -> Result<Option<RefreshTokenModel>, RepositoryError>;
}
