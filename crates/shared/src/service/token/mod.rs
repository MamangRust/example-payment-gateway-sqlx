use crate::{
    abstract_trait::{
        jwt::DynJwtService, refresh_token::command::DynRefreshTokenCommandRepository,
        token::TokenServiceTrait,
    },
    domain::requests::refresh_token::CreateRefreshToken,
    errors::ServiceError,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use tracing::{error, info};

pub struct TokenService {
    token: DynJwtService,
    refresh: DynRefreshTokenCommandRepository,
}

impl TokenService {
    pub fn new(token: DynJwtService, refresh: DynRefreshTokenCommandRepository) -> Self {
        Self { token, refresh }
    }
}

#[async_trait]
impl TokenServiceTrait for TokenService {
    async fn create_access_token(&self, id: i32) -> Result<String, ServiceError> {
        match self.token.generate_token(id as i64, "access") {
            Ok(token) => {
                info!("✅ Successfully generated access token for user_id: {}", id);
                Ok(token)
            }
            Err(e) => {
                error!(
                    "❌ Failed to generate access token for user_id {}: {e:?}",
                    id,
                );
                Err(e)
            }
        }
    }

    async fn create_refresh_token(&self, id: i32) -> Result<String, ServiceError> {
        let token = self.token.generate_token(id as i64, "refresh")?;

        if let Err(e) = self.refresh.delete_by_user_id(id).await {
            error!("❌ Failed to delete existing refresh token: {e:?}");
        }

        let expires_at = (Utc::now() + Duration::hours(24))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let req = CreateRefreshToken {
            user_id: id,
            token: token.clone(),
            expires_at,
        };

        match self.refresh.create(&req).await {
            Ok(_) => {
                info!("✅ Created refresh token for user_id {}", id);
                Ok(token)
            }
            Err(e) => {
                error!("❌ Failed to create refresh token: {e:?}");
                Err(ServiceError::from(e))
            }
        }
    }
}
