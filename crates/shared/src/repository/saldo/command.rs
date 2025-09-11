use crate::{
    abstract_trait::saldo::repository::command::SaldoCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::saldo::{
        CreateSaldoRequest, UpdateSaldoBalance, UpdateSaldoRequest, UpdateSaldoWithdraw,
    },
    errors::RepositoryError,
    model::saldo::SaldoModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct SaldoCommandRepository {
    db: ConnectionPool,
}

impl SaldoCommandRepository {
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
impl SaldoCommandRepositoryTrait for SaldoCommandRepository {
    async fn create(&self, req: &CreateSaldoRequest) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            INSERT INTO saldos (
                card_number,
                total_balance,
                created_at,
                updated_at
            )
            VALUES ($1, $2, NOW(), NOW())
            RETURNING
                saldo_id,
                card_number,
                total_balance as "total_balance!: i64",
                NULL::TIMESTAMP AS "withdraw_time",
                NULL::INT AS "withdraw_amount",
                created_at,
                updated_at,
                deleted_at
            "#,
            req.card_number,
            req.total_balance as i32
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to create saldo: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(saldo)
    }

    async fn update(&self, req: &UpdateSaldoRequest) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            UPDATE saldos
            SET
                card_number = $2,
                total_balance = $3,
                updated_at = NOW()
            WHERE saldo_id = $1 AND deleted_at IS NULL
            RETURNING
                saldo_id,
                card_number,
                total_balance,
                withdraw_amount,
                withdraw_time,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.saldo_id,
            req.card_number,
            req.total_balance as i32
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Saldo not found or deleted: {}", req.saldo_id);
                RepositoryError::NotFound
            }
            _ => {
                error!("❌ Failed to update saldo {}: {e:?}", req.saldo_id);
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(saldo)
    }

    async fn update_balance(
        &self,
        req: &UpdateSaldoBalance,
    ) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            UPDATE saldos
            SET total_balance = $2, updated_at = NOW()
            WHERE card_number = $1 AND deleted_at IS NULL
            RETURNING
                saldo_id,
                card_number,
                total_balance as "total_balance!: i64",
                withdraw_amount,
                withdraw_time,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.card_number,
            req.total_balance as i32
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Saldo not found for card: {}", req.card_number);
                RepositoryError::NotFound
            }
            _ => {
                error!(
                    "❌ Failed to update balance for card {}: {e:?}",
                    req.card_number,
                );
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(saldo)
    }

    async fn update_withdraw(
        &self,
        req: &UpdateSaldoWithdraw,
    ) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            UPDATE saldos
            SET
                withdraw_amount = $2,
                total_balance = total_balance - $2,
                withdraw_time = $3,
                updated_at = NOW()
            WHERE
                card_number = $1
                AND deleted_at IS NULL
                AND total_balance >= $2
            RETURNING
                saldo_id,
                card_number,
                total_balance as "total_balance!: i64",
                $2::INTEGER AS "withdraw_amount!",
                $3::TIMESTAMP AS "withdraw_time!",
                created_at,
                updated_at,
                deleted_at
            "#,
            req.card_number,
            req.withdraw_amount,
            req.withdraw_time
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!(
                    "❌ Insufficient balance or card not found: {}",
                    req.card_number
                );
                RepositoryError::Custom("Insufficient balance or card not found".into())
            }
            _ => {
                error!("❌ Failed to withdraw from card {}: {e:?}", req.card_number);
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(saldo)
    }

    async fn trash(&self, id: i32) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            UPDATE saldos
            SET deleted_at = NOW()
            WHERE saldo_id = $1 AND deleted_at IS NULL
            RETURNING
                saldo_id,
                card_number,
                total_balance as "total_balance!: i64",
                withdraw_amount,
                withdraw_time,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Saldo not found or already trashed: {id}");
                RepositoryError::NotFound
            }
            _ => {
                error!("❌ Failed to trash saldo {id}: {e:?}");
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(saldo)
    }

    async fn restore(&self, id: i32) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let saldo = sqlx::query_as!(
            SaldoModel,
            r#"
            UPDATE saldos
            SET deleted_at = NULL
            WHERE saldo_id = $1 AND deleted_at IS NOT NULL
            RETURNING
                saldo_id,
                card_number,
                total_balance as "total_balance!: i64",
                withdraw_amount,
                withdraw_time,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Saldo not found or not trashed: {id}");
                RepositoryError::NotFound
            }
            _ => {
                error!("❌ Failed to restore saldo {id}: {e:?}");
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(saldo)
    }

    async fn delete_permanent(&self, id: i32) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            DELETE FROM saldos
            WHERE saldo_id = $1 AND deleted_at IS NOT NULL
            "#,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to permanently delete saldo {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(())
    }

    async fn restore_all(&self) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            UPDATE saldos
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to restore all saldos: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(())
    }

    async fn delete_all(&self) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            DELETE FROM saldos
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to delete all trashed saldos: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(())
    }
}
