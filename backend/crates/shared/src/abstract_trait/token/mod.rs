use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::errors::ServiceError;

pub type DynTokenService = Arc<dyn TokenServiceTrait + Send + Sync>;

#[async_trait]
pub trait TokenServiceTrait {
    async fn create_access_token(&self, id: i32) -> Result<String, ServiceError>;
    async fn create_refresh_token(&self, id: i32) -> Result<String, ServiceError>;
}
