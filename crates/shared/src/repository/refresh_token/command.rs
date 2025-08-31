use crate::{
    abstract_trait::refresh_token::command::RefreshTokenCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::refresh_token::{CreateRefreshToken, UpdateRefreshToken},
    errors::RepositoryError,
    model::refresh_token::RefreshTokenModel,
    utils::parse_expiration_datetime,
};
use async_trait::async_trait;
use tracing::{error, info};

pub struct RefreshTokenCommandRepository {
    db: ConnectionPool,
}

impl RefreshTokenCommandRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RefreshTokenCommandRepositoryTrait for RefreshTokenCommandRepository {
    async fn create(
        &self,
        request: &CreateRefreshToken,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let expired_at = match parse_expiration_datetime(&request.expires_at) {
            Ok(datetime) => datetime,
            Err(e) => {
                eprintln!("Failed to parse datetime: {e}");
                return Err(RepositoryError::Custom("Invalid datetime format".into()));
            }
        };

        let result = sqlx::query_as!(
                RefreshTokenModel,
                r#"
                INSERT INTO refresh_tokens (user_id, token, expiration, created_at, updated_at)
                VALUES ($1, $2, $3, current_timestamp, current_timestamp)
                RETURNING refresh_token_id, user_id, token, expiration, created_at, updated_at, deleted_at
                "#,
                request.user_id,
                request.token,
                expired_at,
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!("Failed to create refresh token: {}", e);
                RepositoryError::from(e)
            })?;

        info!("Created refresh token for user_id={}", request.user_id);
        Ok(result)
    }

    async fn update(
        &self,
        request: &UpdateRefreshToken,
    ) -> Result<RefreshTokenModel, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let expired_at = match parse_expiration_datetime(&request.expires_at) {
            Ok(datetime) => datetime,
            Err(e) => {
                eprintln!("Failed to parse datetime: {e}");
                return Err(RepositoryError::Custom("Invalid datetime format".into()));
            }
        };

        let updated = sqlx::query_as!(
                RefreshTokenModel,
                r#"
                UPDATE refresh_tokens
                SET token = $2, expiration = $3, updated_at = current_timestamp
                WHERE user_id = $1 AND deleted_at IS NULL
                RETURNING refresh_token_id, user_id, token, expiration, created_at, updated_at, deleted_at
                "#,
                request.user_id,
                request.token,
                expired_at
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!(
                    "Failed to update refresh token for user_id={}: {}",
                    request.user_id, e
                );
                RepositoryError::from(e)
            })?;

        info!("Updated refresh token for user_id={}", request.user_id);
        Ok(updated)
    }

    async fn delete_token(&self, token: String) -> Result<(), RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens
                WHERE token = $1
                "#,
            token
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("Failed to delete token={}: {}", token, e);
            RepositoryError::from(e)
        })?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        info!("Deleted refresh token");
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: i32) -> Result<(), RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens
                WHERE user_id = $1
                "#,
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("Failed to delete tokens for user_id={}: {}", user_id, e);
            RepositoryError::from(e)
        })?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        info!("Deleted all refresh tokens for user_id={}", user_id);
        Ok(())
    }
}
