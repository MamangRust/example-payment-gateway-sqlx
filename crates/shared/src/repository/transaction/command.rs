use crate::{
    abstract_trait::transaction::repository::command::TransactionCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::transaction::{
        CreateTransactionRequest, UpdateTransactionRequest, UpdateTransactionStatus,
    },
    errors::RepositoryError,
    model::transaction::TransactionModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct TransactionCommandRepository {
    db: ConnectionPool,
}

impl TransactionCommandRepository {
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
impl TransactionCommandRepositoryTrait for TransactionCommandRepository {
    async fn create(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransactionModel,
            r#"
        INSERT INTO transactions (
            card_number,
            amount,
            payment_method,
            merchant_id,
            transaction_time,
            status,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, 'pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING
            transaction_id ,
            card_number,
            transaction_no,
            amount as "amount!",
            payment_method,
            merchant_id,
            transaction_time,
            status,
            created_at,
            updated_at,
            deleted_at
        "#,
            req.card_number,
            req.amount as i64,
            req.payment_method,
            req.merchant_id,
            req.transaction_time
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transaction creation: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update(
        &self,
        req: &UpdateTransactionRequest,
    ) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransactionModel,
            r#"
            UPDATE transactions
            SET
                card_number = $2,
                amount = $3,
                payment_method = $4,
                merchant_id = $5,
                transaction_time = $6,
                updated_at = CURRENT_TIMESTAMP
            WHERE transaction_id = $1 AND deleted_at IS NULL
            RETURNING
                transaction_id,
                card_number,
                transaction_no,
                amount as "amount!",
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.transaction_id,
            req.card_number,
            req.amount as i64,
            req.payment_method,
            req.merchant_id,
            req.transaction_time
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transaction update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn update_status(
        &self,
        req: &UpdateTransactionStatus,
    ) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransactionModel,
            r#"
            UPDATE transactions
            SET
                status = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE transaction_id = $1 AND deleted_at IS NULL
            RETURNING
                transaction_id,
                card_number,
                transaction_no,
                amount as "amount!",
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.transaction_id,
            req.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error during transaction status update: {e:?}");
            match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Sqlx(e),
            }
        })?;

        Ok(record)
    }

    async fn trashed(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransactionModel,
            r#"
            UPDATE transactions
            SET deleted_at = CURRENT_TIMESTAMP
            WHERE transaction_id = $1 AND deleted_at IS NULL
            RETURNING
                transaction_id,
                card_number,
                transaction_no,
                amount as "amount!",
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            transaction_id
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

    async fn restore(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            TransactionModel,
            r#"
            UPDATE transactions
            SET deleted_at = NULL
            WHERE transaction_id = $1 AND deleted_at IS NOT NULL
            RETURNING
                transaction_id,
                card_number,
                transaction_no,
                amount as "amount!",
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            transaction_id
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

    async fn delete_permanent(&self, transaction_id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            DELETE FROM transactions
            WHERE transaction_id = $1 AND deleted_at IS NOT NULL
            "#,
            transaction_id
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
            UPDATE transactions
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
            DELETE FROM transactions
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
