use crate::{
    domain::{
        requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
        responses::{ApiResponse, ApiResponsePagination, TopupResponse, TopupResponseDeleteAt},
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TopupQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, HttpError>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, HttpError>;
    async fn find_active(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, HttpError>;
    async fn find_trashed(
        &self,
        req: &FindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, HttpError>;
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, HttpError>;
    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, HttpError>;
}
