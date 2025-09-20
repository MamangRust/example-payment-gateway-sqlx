use crate::{
    abstract_trait::card::repository::command::CardCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::card::{CreateCardRequest, UpdateCardRequest},
    errors::RepositoryError,
    model::card::CardModel,
    utils::random_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct CardCommandRepository {
    db: ConnectionPool,
}

impl CardCommandRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }

    async fn get_conn(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, RepositoryError> {
        self.db.acquire().await.map_err(|e| {
            error!("❌ Failed to acquire DB connection: {e:?}");
            RepositoryError::from(e)
        })
    }
}

#[async_trait]
impl CardCommandRepositoryTrait for CardCommandRepository {
    async fn create(&self, request: &CreateCardRequest) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let card_number = random_card_number()
            .map_err(|_| RepositoryError::Custom("❌ error ketika gen card_number".to_string()))?;

        let card = sqlx::query_as!(
            CardModel,
            r#"
            INSERT INTO cards (
                user_id,
                card_number,
                card_type,
                expire_date,
                cvv,
                card_provider,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING 
                card_id,
                user_id,
                card_number,
                card_type,
                expire_date,
                cvv,
                card_provider,
                created_at,
                updated_at,
                deleted_at
            "#,
            request.user_id,
            card_number,
            request.card_type,
            request.expire_date,
            request.cvv,
            request.card_provider
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to create card: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(card)
    }

    async fn update(&self, request: &UpdateCardRequest) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let card_id = request
            .card_id
            .ok_or_else(|| RepositoryError::Custom("card_id is required".into()))?;

        let card = sqlx::query_as!(
            CardModel,
            r#"
            UPDATE cards
            SET
                card_type = COALESCE($2, card_type),
                expire_date = COALESCE($3, expire_date),
                cvv = COALESCE($4, cvv),
                card_provider = COALESCE($5, card_provider),
                updated_at = NOW()
            WHERE
                card_id = $1
                AND deleted_at IS NULL
            RETURNING 
                card_id,
                user_id,
                card_number,
                card_type,
                expire_date,
                cvv,
                card_provider,
                created_at,
                updated_at,
                deleted_at
            "#,
            card_id,
            request.card_type,
            request.expire_date,
            request.cvv,
            request.card_provider
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to create update: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(card)
    }

    async fn trash(&self, id: i32) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let card = sqlx::query_as!(
            CardModel,
            r#"
            UPDATE cards
            SET deleted_at = NOW()
            WHERE card_id = $1 AND deleted_at IS NULL
            RETURNING 
                card_id,
                user_id,
                card_number,
                card_type,
                expire_date,
                cvv,
                card_provider,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to trash card: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(card)
    }

    async fn restore(&self, id: i32) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let card = sqlx::query_as!(
            CardModel,
            r#"
            UPDATE cards
            SET deleted_at = NULL
            WHERE card_id = $1 AND deleted_at IS NOT NULL
            RETURNING 
                card_id,
                user_id,
                card_number,
                card_type,
                expire_date,
                cvv,
                card_provider,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to restore card: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(card)
    }

    async fn delete_permanent(&self, id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM cards
            WHERE card_id = $1 AND deleted_at IS NOT NULL
            "#,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to permanently delete card ID {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            UPDATE cards
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to restore all merchant: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM cards
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to delete all merchant: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }
}
