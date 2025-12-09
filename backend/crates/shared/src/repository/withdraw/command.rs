use crate::{
    abstract_trait::withdraw::repository::command::WithdrawCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::withdraw::{
        CreateWithdrawRequest, UpdateWithdrawRequest, UpdateWithdrawStatus,
    },
    errors::RepositoryError,
    model::withdraw::WithdrawModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct WithdrawCommandRepository {
    db: ConnectionPool,
}

impl WithdrawCommandRepository {
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
impl WithdrawCommandRepositoryTrait for WithdrawCommandRepository {
    async fn create(&self, req: &CreateWithdrawRequest) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let now = chrono::Utc::now().naive_utc();

        let record = sqlx::query_as!(
            WithdrawModel,
            r#"
            INSERT INTO withdraws (
                card_number,
                withdraw_amount,
                withdraw_time,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount as "withdraw_amount!",
                status,
                withdraw_time,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.card_number,
            req.withdraw_amount as i64,
            req.withdraw_time,
            now,
            now
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in create withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update(&self, req: &UpdateWithdrawRequest) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let withdraw_id = req
            .withdraw_id
            .ok_or_else(|| RepositoryError::Custom("withdraw_id is required".into()))?;

        let record = sqlx::query_as!(
            WithdrawModel,
            r#"
            UPDATE withdraws
            SET
                card_number = $2,
                withdraw_amount = $3,
                withdraw_time = $4,
                updated_at = current_timestamp
            WHERE
                withdraw_id = $1
                AND deleted_at IS NULL
            RETURNING
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount as "withdraw_amount!",
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            withdraw_id,
            req.card_number,
            req.withdraw_amount as i64,
            req.withdraw_time
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in update withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update_status(
        &self,
        req: &UpdateWithdrawStatus,
    ) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            WithdrawModel,
            r#"
            UPDATE withdraws
            SET
                status = $2,
                updated_at = current_timestamp
            WHERE
                withdraw_id = $1
                AND deleted_at IS NULL
            RETURNING
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount as "withdraw_amount!",
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.withdraw_id,
            req.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in update_status withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn trashed(&self, withdraw_id: i32) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            WithdrawModel,
            r#"
            UPDATE withdraws
            SET
                deleted_at = current_timestamp
            WHERE
                withdraw_id = $1
                AND deleted_at IS NULL
            RETURNING
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount as "withdraw_amount!",
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            withdraw_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in trashed withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn restore(&self, withdraw_id: i32) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            WithdrawModel,
            r#"
            UPDATE withdraws
            SET
                deleted_at = NULL
            WHERE
                withdraw_id = $1
                AND deleted_at IS NOT NULL
            RETURNING
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount as "withdraw_amount!",
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            withdraw_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in restore withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn delete_permanent(&self, withdraw_id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query(
            r#"
            DELETE FROM withdraws
            WHERE
                withdraw_id = $1
                AND deleted_at IS NOT NULL
            "#,
        )
        .bind(withdraw_id)
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in delete_permanent withdraw: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result =
            sqlx::query("UPDATE withdraws SET deleted_at = NULL WHERE deleted_at IS NOT NULL")
                .execute(&mut *conn)
                .await
                .map_err(|e| {
                    error!("❌ Database error in restore_all withdraw: {e:?}");
                    RepositoryError::Sqlx(e)
                })?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query("DELETE FROM withdraws WHERE deleted_at IS NOT NULL")
            .execute(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in delete_all withdraw: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok(result.rows_affected() > 0)
    }
}
