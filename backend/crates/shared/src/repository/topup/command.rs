use crate::{
    abstract_trait::topup::repository::command::TopupCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::topup::{
        CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest, UpdateTopupStatus,
    },
    errors::RepositoryError,
    model::topup::TopupModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct TopupCommandRepository {
    db: ConnectionPool,
}

impl TopupCommandRepository {
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
impl TopupCommandRepositoryTrait for TopupCommandRepository {
    async fn create(&self, req: &CreateTopupRequest) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let topup_time = chrono::Utc::now().naive_utc();

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            INSERT INTO topups (
                card_number,
                topup_amount,
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, 'pending',CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.card_number,
            req.topup_amount as i64,
            req.topup_method,
            topup_time
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during topup creation: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update(&self, req: &UpdateTopupRequest) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let topup_id = req
            .topup_id
            .ok_or_else(|| RepositoryError::Custom("topup_id is required".into()))?;

        let topup_time = chrono::Utc::now().naive_utc();

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            UPDATE topups
            SET
                card_number = $2,
                topup_amount = $3,
                topup_method = $4,
                topup_time = $5,
                updated_at = CURRENT_TIMESTAMP
            WHERE topup_id = $1 AND deleted_at IS NULL
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            topup_id,
            req.card_number,
            req.topup_amount as i64,
            req.topup_method,
            topup_time,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during topup update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn update_amount(&self, req: &UpdateTopupAmount) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            UPDATE topups
            SET
                topup_amount = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE topup_id = $1 AND deleted_at IS NULL
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.topup_id,
            req.topup_amount as i64
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during topup amount update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn update_status(&self, req: &UpdateTopupStatus) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            UPDATE topups
            SET
                status = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE topup_id = $1 AND deleted_at IS NULL
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.topup_id,
            req.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during topup status update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn trashed(&self, topup_id: i32) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            UPDATE topups
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE topup_id = $1 AND deleted_at IS NULL
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            topup_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during soft delete (trash): {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn restore(&self, topup_id: i32) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TopupModel,
            r#"
            UPDATE topups
            SET deleted_at = NULL
            WHERE topup_id = $1 AND deleted_at IS NOT NULL
            RETURNING
                topup_id,
                card_number,
                topup_no,
                topup_amount as "topup_amount!",
                topup_method,
                topup_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            topup_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during restore: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn delete_permanent(&self, topup_id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            DELETE FROM topups
            WHERE topup_id = $1 AND deleted_at IS NOT NULL
            "#,
            topup_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during permanent delete: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(true)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            UPDATE topups
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during restore all: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM topups
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during delete all permanent: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }
}
