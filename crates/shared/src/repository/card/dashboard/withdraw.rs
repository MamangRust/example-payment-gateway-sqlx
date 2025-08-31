use crate::{
    abstract_trait::card::repository::CardDashboardWithdrawRepositoryTrait, config::ConnectionPool,
    errors::RepositoryError,
};
use async_trait::async_trait;

pub struct CardDashboardWithdrawRepository {
    db: ConnectionPool,
}

impl CardDashboardWithdrawRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardDashboardWithdrawRepositoryTrait for CardDashboardWithdrawRepository {
    async fn get_total_amount(&self) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(s.withdraw_amount), 0) AS "total_withdraw_amount!"
            FROM withdraws s
            JOIN cards c ON s.card_number = c.card_number
            WHERE s.deleted_at IS NULL 
              AND c.deleted_at IS NULL
            "#
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(total)
    }

    async fn get_total_amount_by_card(&self, card_number: String) -> Result<i64, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(s.withdraw_amount), 0) AS "total_withdraw_amount!"
            FROM withdraws s
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

        Ok(total)
    }
}
