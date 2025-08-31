use crate::{
    domain::requests::card::{CreateCardRequest, UpdateCardRequest},
    errors::RepositoryError,
    model::card::CardModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardCommandRepository = Arc<dyn CardCommandRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardCommandRepositoryTrait {
    async fn create(&self, request: &CreateCardRequest) -> Result<CardModel, RepositoryError>;
    async fn update(&self, request: &UpdateCardRequest) -> Result<CardModel, RepositoryError>;
    async fn trash(&self, id: i32) -> Result<CardModel, RepositoryError>;
    async fn restore(&self, id: i32) -> Result<CardModel, RepositoryError>;
    async fn delete_permanent(&self, id: i32) -> Result<(), RepositoryError>;
    async fn restore_all(&self) -> Result<(), RepositoryError>;
    async fn delete_all(&self) -> Result<(), RepositoryError>;
}
