use crate::errors::ServiceError;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynJwtService = Arc<dyn JwtServiceTrait + Send + Sync>;

#[async_trait]
pub trait JwtServiceTrait: Send + Sync + std::fmt::Debug {
    fn generate_token(&self, user_id: i64, token_type: &str) -> Result<String, ServiceError>;
    fn verify_token(&self, token: &str, expected_type: &str) -> Result<i64, ServiceError>;
}
