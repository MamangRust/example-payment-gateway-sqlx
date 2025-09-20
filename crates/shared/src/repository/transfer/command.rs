use crate::{
    abstract_trait::transfer::repository::command::TransferCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::transfer::{
        CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest,
        UpdateTransferStatus,
    },
    errors::RepositoryError,
    model::transfer::TransferModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct TransferCommandRepository {
    db: ConnectionPool,
}

impl TransferCommandRepository {
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
impl TransferCommandRepositoryTrait for TransferCommandRepository {
    async fn create(&self, req: &CreateTransferRequest) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let now = chrono::Utc::now().naive_utc();

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            INSERT INTO transfers (
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.transfer_from,
            req.transfer_to,
            req.transfer_amount as i64,
            now
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transfer creation: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update(&self, req: &UpdateTransferRequest) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let transfer_id = req
            .transfer_id
            .ok_or_else(|| RepositoryError::Custom("transfer_id is required".into()))?;

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            UPDATE transfers
            SET
                transfer_from = $2,
                transfer_to = $3,
                transfer_amount = $4,
                transfer_time = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE transfer_id = $1 AND deleted_at IS NULL
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            transfer_id,
            req.transfer_from,
            req.transfer_to,
            req.transfer_amount as i64
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transfer update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn update_amount(
        &self,
        req: &UpdateTransferAmountRequest,
    ) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            UPDATE transfers
            SET
                transfer_amount = $2,
                transfer_time = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE transfer_id = $1 AND deleted_at IS NULL
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.transfer_id,
            req.transfer_amount as i64
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transfer amount update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn update_status(
        &self,
        req: &UpdateTransferStatus,
    ) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            UPDATE transfers
            SET
                status = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE transfer_id = $1 AND deleted_at IS NULL
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.transfer_id,
            req.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transfer status update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn trashed(&self, transfer_id: i32) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            UPDATE transfers
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE transfer_id = $1 AND deleted_at IS NULL
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            transfer_id
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

    async fn restore(&self, transfer_id: i32) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransferModel,
            r#"
            UPDATE transfers
            SET deleted_at = NULL
            WHERE transfer_id = $1 AND deleted_at IS NOT NULL
            RETURNING
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount as "transfer_amount!",
                transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            transfer_id
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

    async fn delete_permanent(&self, transfer_id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM transfers
            WHERE transfer_id = $1 AND deleted_at IS NOT NULL
            "#,
            transfer_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during permanent delete: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            UPDATE transfers
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
            DELETE FROM transfers
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
