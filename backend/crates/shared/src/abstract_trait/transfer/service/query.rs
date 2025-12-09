use crate::{
    domain::{
        requests::transfer::FindAllTransfers,
        responses::{
            ApiResponse, ApiResponsePagination, TransferResponse, TransferResponseDeleteAt,
        },
    },
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferQueryService = Arc<dyn TransferQueryServiceTrait + Send + Sync>;

#[async_trait]
pub trait TransferQueryServiceTrait {
    async fn find_all(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, ServiceError>;

    async fn find_by_id(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, ServiceError>;

    async fn find_by_active(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError>;

    async fn find_by_trashed(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, ServiceError>;

    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError>;

    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, ServiceError>;
}
