use crate::{
    abstract_trait::card::repository::CardDashboardBalanceRepositoryTrait, config::ConnectionPool,
    errors::RepositoryError,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct CardDashboardBalanceRepository {
    db: ConnectionPool,
}

impl CardDashboardBalanceRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardDashboardBalanceRepositoryTrait for CardDashboardBalanceRepository {
    async fn get_total_balance(&self) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let total: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT SUM(s.total_balance) AS total_balance
            FROM saldos s
            JOIN cards c ON s.card_number = c.card_number
            WHERE s.deleted_at IS NULL AND c.deleted_at IS NULL
            "#
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(total.unwrap_or(0))
    }

    async fn get_total_balance_by_card(&self, card_number: String) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let total: Option<i64> = sqlx::query_scalar!(
            r#"
            SELECT SUM(s.total_balance) AS total_balance
            FROM saldos s
            JOIN cards c ON s.card_number = c.card_number
            WHERE s.deleted_at IS NULL 
              AND c.deleted_at IS NULL 
              AND c.card_number = $1
            "#,
            card_number
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(total.unwrap_or(0))
    }
}
