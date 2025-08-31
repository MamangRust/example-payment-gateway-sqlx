use crate::{
    domain::{
        requests::transfer::FindAllTransfers,
        responses::{
            ApiResponse, ApiResponsePagination, TransferResponse, TransferResponseDeleteAt,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynTransferQueryGrpcClient = Arc<dyn TransferQueryGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait TransferQueryGrpcClientTrait {
    async fn find_all(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, AppErrorHttp>;

    async fn find_by_id(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp>;

    async fn find_by_active(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, AppErrorHttp>;

    async fn find_by_trashed(
        &self,
        req: &FindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, AppErrorHttp>;

    async fn find_by_transfer_from(
        &self,
        transfer_from: String,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, AppErrorHttp>;

    async fn find_by_transfer_to(
        &self,
        transfer_to: String,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, AppErrorHttp>;
}
