use crate::{
    abstract_trait::card::repository::CardDashboardTransactionRepositoryTrait,
    config::ConnectionPool, errors::RepositoryError,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct CardDashboardTransactionRepository {
    db: ConnectionPool,
}

impl CardDashboardTransactionRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardDashboardTransactionRepositoryTrait for CardDashboardTransactionRepository {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COALESCE(SUM(t.amount), 0) AS total_transaction_amount
            FROM transactions t
            JOIN cards c ON t.card_number = c.card_number
            WHERE t.deleted_at IS NULL AND c.deleted_at IS NULL
            "#,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(result)
    }

    async fn get_total_amount_by_card(&self, card_number: String) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COALESCE(SUM(t.amount), 0) AS total_transaction_amount
            FROM transactions t
            JOIN cards c ON t.card_number = c.card_number
            WHERE t.deleted_at IS NULL 
              AND c.deleted_at IS NULL 
              AND c.card_number = $1
            "#,
        )
        .bind(card_number)
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(result)
    }
}
