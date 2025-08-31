use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        requests::auth::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::ServiceError,
};

pub type DynAuthService = Arc<dyn AuthServiceTrait + Send + Sync>;

#[async_trait]
pub trait AuthServiceTrait {
    async fn register_user(
        &self,
        input: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError>;
    async fn login_user(&self, input: &AuthRequest) -> Result<ApiResponse<String>, ServiceError>;
    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, ServiceError>;
    async fn refresh_token(&self, token: &str) -> Result<ApiResponse<TokenResponse>, ServiceError>;
}
