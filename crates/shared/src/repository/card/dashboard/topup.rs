use crate::{
    abstract_trait::card::repository::dashboard::topup::CardDashboardTopupRepositoryTrait,
    config::ConnectionPool, errors::RepositoryError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardDashboardTopupRepository {
    db: ConnectionPool,
}

impl CardDashboardTopupRepository {
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
impl CardDashboardTopupRepositoryTrait for CardDashboardTopupRepository {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let total: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT SUM(t.topup_amount) AS total_topup_amount
            FROM topups t
            JOIN cards c ON t.card_number = c.card_number
            WHERE t.deleted_at IS NULL AND c.deleted_at IS NULL
            "#
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in get_total_amount: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(total.unwrap_or(0))
    }
    async fn get_total_amount_by_card(&self, card_number: String) -> Result<i64, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let total: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT SUM(t.topup_amount) AS total_topup_amount
            FROM topups t
            JOIN cards c ON t.card_number = c.card_number
            WHERE t.deleted_at IS NULL 
              AND c.deleted_at IS NULL 
              AND c.card_number = $1
            "#,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in get_total_amount_by_card: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(total.unwrap_or(0))
    }
}
