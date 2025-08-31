use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::errors::ServiceError;

pub type DynHashing = Arc<dyn HashingTrait + Send + Sync>;

#[async_trait]
pub trait HashingTrait {
    async fn hash_password(&self, password: &str) -> Result<String, ServiceError>;
    async fn compare_password(
        &self,
        hashed_password: &str,
        password: &str,
    ) -> Result<(), ServiceError>;
}
