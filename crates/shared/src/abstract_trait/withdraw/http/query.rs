use crate::{
    domain::{
        requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawResponse, WithdrawResponseDeleteAt,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynWithdrawQueryGrpcClient = Arc<dyn WithdrawQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait WithdrawQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, AppErrorHttp>;
    async fn find_all_by_card_number(
        &self,
        req: &FindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, AppErrorHttp>;
    async fn find_by_id(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, AppErrorHttp>;
    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_trashed(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, AppErrorHttp>;
}
