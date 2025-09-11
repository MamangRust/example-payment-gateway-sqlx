use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        requests::auth::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::AppErrorHttp,
};

pub type DynAuthGrpcClient = Arc<dyn AuthGrpcClientTrait + Send + Sync>;

#[async_trait]
pub trait AuthGrpcClientTrait {
    async fn login(
        &self,
        request: &AuthRequest,
    ) -> Result<ApiResponse<TokenResponse>, AppErrorHttp>;
    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;
    async fn refresh_token(&self, token: &str) -> Result<ApiResponse<TokenResponse>, AppErrorHttp>;
    async fn register(
        &self,
        request: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;
}
