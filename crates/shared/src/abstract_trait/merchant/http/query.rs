use crate::{
    domain::{
        requests::merchant::FindAllMerchants,
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
        },
    },
    errors::AppErrorHttp,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MerchantQueryGrpcClientTrait {
    async fn find_all(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, AppErrorHttp>;
    async fn find_active(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, AppErrorHttp>;
    async fn find_trashed(
        &self,
        request: &FindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, AppErrorHttp>;
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp>;
    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp>;
    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, AppErrorHttp>;
}
