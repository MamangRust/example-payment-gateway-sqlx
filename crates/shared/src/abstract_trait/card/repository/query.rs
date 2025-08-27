use crate::{
    domain::{
        requests::FindAllCards,
        responses::{ApiResponse, ApiResponsePagination, CardResponse},
    },
    errors::{RepositoryError, ServiceError},
    model::card::CardModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardQueryRepository = Arc<dyn CardQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardQueryRepositoryTrait {
    async fn find_all(&self, request: FindAllCards) -> Result<Vec<CardModel>, RepositoryError>;
    async fn find_active(&self, request: FindAllCards) -> Result<Vec<CardModel>, RepositoryError>;
    async fn find_trashed(&self, request: FindAllCards) -> Result<Vec<CardModel>, RepositoryError>;
    async fn find_by_id(&self, id: String) -> Result<CardModel, RepositoryError>;
}
