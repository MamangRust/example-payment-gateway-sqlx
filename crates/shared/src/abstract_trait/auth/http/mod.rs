use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        requests::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::AppErrorHttp,
};

pub type DynAuthService = Arc<dyn AuthServiceTrait + Send + Sync>;

#[async_trait]
pub trait AuthServiceTrait {
    async fn login(
        &self,
        &request: AuthRequest,
    ) -> Result<ApiResponse<TokenResponse>, AppErrorHttp>;
    async fn get_me(&self) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;
    async fn refresh_token(&self) -> Result<ApiResponse<TokenResponse>, AppErrorHttp>;
    async fn register(
        &self,
        &request: RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp>;
}
