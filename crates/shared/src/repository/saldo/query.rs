use crate::{
    abstract_trait::saldo::repository::query::SaldoQueryRepositoryTrait, config::ConnectionPool,
    domain::requests::saldo::FindAllSaldos, errors::RepositoryError, model::saldo::SaldoModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct SaldoQueryRepository {
    db: ConnectionPool,
}

impl SaldoQueryRepository {
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
impl SaldoQueryRepositoryTrait for SaldoQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let rows = sqlx::query!(
            r#"
            SELECT 
                saldo_id, 
                card_number, 
                total_balance,
                withdraw_amount, 
                withdraw_time,
                created_at, 
                updated_at, 
                deleted_at, 
                COUNT(*) OVER() AS total_count
            FROM saldos
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR card_number ILIKE '%' || $1 || '%')
            ORDER BY saldo_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch saldos: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| SaldoModel {
                saldo_id: r.saldo_id,
                card_number: r.card_number,
                withdraw_amount: r.withdraw_amount,
                total_balance: r.total_balance as i64,
                withdraw_time: r.withdraw_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }

    async fn find_active(
        &self,
        request: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = request.page_size.clamp(1, 100);
        let offset = (request.page - 1).max(0) * limit;

        let search_pattern = if request.search.trim().is_empty() {
            None
        } else {
            Some(request.search.as_str())
        };

        let rows = sqlx::query!(
            r#"
            SELECT 
                saldo_id, 
                card_number, 
                total_balance,
                withdraw_amount, 
                withdraw_time,
                created_at, 
                updated_at, 
                deleted_at, 
                COUNT(*) OVER() AS total_count
            FROM saldos
            WHERE deleted_at IS NULL  
                AND ($1::TEXT IS NULL OR card_number ILIKE '%' || $1 || '%')
            ORDER BY saldo_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch active saldos: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| SaldoModel {
                saldo_id: r.saldo_id,
                card_number: r.card_number,
                withdraw_amount: r.withdraw_amount,
                total_balance: r.total_balance as i64,
                withdraw_time: r.withdraw_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }

    async fn find_trashed(
        &self,
        req: &FindAllSaldos,
    ) -> Result<(Vec<SaldoModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let rows = sqlx::query!(
            r#"
            SELECT 
                saldo_id, 
                card_number, 
                total_balance,
                withdraw_amount, 
                withdraw_time,
                created_at, 
                updated_at, 
                deleted_at, 
                COUNT(*) OVER() AS total_count
            FROM saldos
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL OR card_number ILIKE '%' || $1 || '%')
            ORDER BY saldo_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch trashed saldos: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| SaldoModel {
                saldo_id: r.saldo_id,
                card_number: r.card_number,
                withdraw_amount: r.withdraw_amount,
                total_balance: r.total_balance as i64,
                withdraw_time: r.withdraw_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }
    async fn find_by_card(&self, card_number: &str) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT 
                saldo_id, 
                card_number, 
                total_balance,
                withdraw_amount, 
                withdraw_time,
                created_at, 
                updated_at, 
                deleted_at
            FROM saldos
            WHERE card_number = $1 AND deleted_at IS NULL
        "#,
            card_number
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to query saldo by card_number: {e:?}");
            RepositoryError::from(e)
        })?;

        match row {
            Some(r) => Ok(SaldoModel {
                saldo_id: r.saldo_id,
                card_number: r.card_number,
                withdraw_amount: r.withdraw_amount,
                total_balance: r.total_balance as i64,
                withdraw_time: r.withdraw_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            }),
            None => {
                error!("❌ Saldo with card_number {card_number} not found");
                Err(RepositoryError::NotFound)
            }
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<SaldoModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT 
                saldo_id, 
                card_number, 
                total_balance,
                withdraw_amount, 
                withdraw_time,
                created_at, 
                updated_at, 
                deleted_at
            FROM saldos
            WHERE saldo_id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to query saldo by ID: {e:?}");
            RepositoryError::from(e)
        })?;

        match row {
            Some(r) => Ok(SaldoModel {
                saldo_id: r.saldo_id,
                card_number: r.card_number,
                withdraw_amount: r.withdraw_amount,
                total_balance: r.total_balance as i64,
                withdraw_time: r.withdraw_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            }),
            None => {
                error!("❌ Saldo with ID {id} not found");
                Err(RepositoryError::NotFound)
            }
        }
    }
}
