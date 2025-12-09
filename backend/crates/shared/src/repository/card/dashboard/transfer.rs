use crate::{
    abstract_trait::card::repository::dashboard::transfer::CardDashboardTransferRepositoryTrait,
    config::ConnectionPool, errors::RepositoryError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct CardDashboardTransferRepository {
    db: ConnectionPool,
}

impl CardDashboardTransferRepository {
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
impl CardDashboardTransferRepositoryTrait for CardDashboardTransferRepository {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(transfer_amount), 0) AS "total_transfer_amount!"
            FROM transfers
            WHERE deleted_at IS NULL
            "#
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("Database error in get_total_amount: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(total)
    }

    async fn get_total_amount_by_sender(
        &self,
        card_number: String,
    ) -> Result<i64, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(transfer_amount), 0) AS "total_transfer_amount!"
            FROM transfers
            WHERE transfer_from = $1
              AND deleted_at IS NULL
            "#,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in get_total_amount_by_sender: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(total)
    }

    async fn get_total_amount_by_receiver(
        &self,
        card_number: String,
    ) -> Result<i64, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(transfer_amount), 0) AS "total_transfer_amount!"
            FROM transfers
            WHERE transfer_to = $1
              AND deleted_at IS NULL
            "#,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in get_total_amount_by_receiver: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(total)
    }
}
