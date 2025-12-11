use crate::{
    domain::{
        requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawResponse, WithdrawResponseDeleteAt,
        },
    },
    errors::HttpError,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WithdrawQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, HttpError>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, HttpError>;
    async fn find_by_id(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, HttpError>;
    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, HttpError>;
    async fn find_by_trashed(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, HttpError>;
}
