use crate::{
    domain::requests::card::FindAllCards, errors::RepositoryError, model::card::CardModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardQueryRepository = Arc<dyn CardQueryRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait CardQueryRepositoryTrait {
    async fn find_all(
        &self,
        request: &FindAllCards,
    ) -> Result<(Vec<CardModel>, i64), RepositoryError>;
    async fn find_active(
        &self,
        request: &FindAllCards,
    ) -> Result<(Vec<CardModel>, i64), RepositoryError>;
    async fn find_trashed(
        &self,
        request: &FindAllCards,
    ) -> Result<(Vec<CardModel>, i64), RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<CardModel, RepositoryError>;
    async fn find_card_by_card_number(
        &self,
        card_number: String,
    ) -> Result<CardModel, RepositoryError>;
    async fn find_card_by_user_id(&self, user_id: i32) -> Result<Vec<CardModel>, RepositoryError>;
}
