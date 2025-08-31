use crate::{
    domain::{
        requests::card::FindAllCards,
        responses::{ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt},
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynCardQueryGrpcClient = Arc<dyn CardQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait CardQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, AppErrorHttp>;
    async fn find_active(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, AppErrorHttp>;
    async fn find_trashed(
        &self,
        req: &FindAllCards,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, AppErrorHttp>;
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<CardResponse>>, AppErrorHttp>;
    async fn find_by_card_number(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp>;
}
