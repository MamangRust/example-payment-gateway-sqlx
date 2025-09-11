use crate::{
    domain::{
        requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
        responses::{ApiResponse, ApiResponsePagination, TopupResponse, TopupResponseDeleteAt},
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTopupQueryService = Arc<dyn TopupQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait TopupQueryServiceTrait {
    async fn find_all(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, ServiceError>;
    async fn find_active(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError>;
    async fn find_trashed(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, ServiceError>;
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, ServiceError>;
    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, ServiceError>;
}
