use crate::{
    domain::{
        requests::FindAllCards,
        responses::{ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt},
    },
    errors::{RepositoryError, ServiceError},
    model::card::CardModel,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait CardQueryServiceTrait {
    async fn find_all(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, ServiceError>;
    async fn find_active(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError>;
    async fn find_trashed(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, ServiceError>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, ServiceError>;
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<CardResponse>>, ServiceError>;
}
