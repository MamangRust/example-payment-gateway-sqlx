use crate::{
    abstract_trait::transaction::repository::query::TransactionQueryRepositoryTrait,
    config::ConnectionPool,
    domain::requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
    errors::RepositoryError,
    model::transaction::TransactionModel,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use tracing::error;

pub struct TransactionQueryRepository {
    db: ConnectionPool,
}

impl TransactionQueryRepository {
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
impl TransactionQueryRepositoryTrait for TransactionQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let sql = r#"
            SELECT
                transaction_id,
                card_number,
                transaction_no,
                amount,
                payment_method,
                merchant_id,
                status,
                transaction_time,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transactions
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL
                   OR card_number ILIKE '%' || $1 || '%'
                   OR payment_method ILIKE '%' || $1 || '%'
                   OR status ILIKE '%' || $1 || '%')
            ORDER BY transaction_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all transactions: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransactionModel {
                    transaction_id: row.try_get("transaction_id")?,
                    card_number: row.try_get("card_number")?,
                    transaction_no: row.try_get("transaction_no")?,
                    amount: row.try_get("amount")?,
                    payment_method: row.try_get("payment_method")?,
                    merchant_id: row.try_get("merchant_id")?,
                    transaction_time: row.try_get("transaction_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transaction rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let sql = r#"
            SELECT
                transaction_id,
                card_number,
                transaction_no,
                amount,
                payment_method,
                merchant_id,
                status,
                transaction_time,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transactions
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL
                   OR card_number ILIKE '%' || $1 || '%'
                   OR payment_method ILIKE '%' || $1 || '%'
                   OR status ILIKE '%' || $1 || '%')
            ORDER BY transaction_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all_active transactions: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransactionModel {
                    transaction_id: row.try_get("transaction_id")?,
                    card_number: row.try_get("card_number")?,
                    transaction_no: row.try_get("transaction_no")?,
                    amount: row.try_get("amount")?,
                    payment_method: row.try_get("payment_method")?,
                    merchant_id: row.try_get("merchant_id")?,
                    transaction_time: row.try_get("transaction_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transaction rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let sql = r#"
            SELECT
                transaction_id,
                card_number,
                transaction_no,
                amount,
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transactions
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL
                   OR card_number ILIKE '%' || $1 || '%'
                   OR payment_method ILIKE '%' || $1 || '%')
            ORDER BY transaction_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_trashed transactions: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransactionModel {
                    transaction_id: row.try_get("transaction_id")?,
                    card_number: row.try_get("card_number")?,
                    transaction_no: row.try_get("transaction_no")?,
                    amount: row.try_get("amount")?,
                    payment_method: row.try_get("payment_method")?,
                    merchant_id: row.try_get("merchant_id")?,
                    transaction_time: row.try_get("transaction_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map trashed transaction rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<(Vec<TransactionModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let limit = req.page_size.clamp(1, 100);
        let offset = (req.page - 1).max(0) * limit;

        let search_pattern = if req.search.trim().is_empty() {
            None
        } else {
            Some(req.search.as_str())
        };

        let sql = r#"
            SELECT
                transaction_id,
                card_number,
                transaction_no,
                amount,
                payment_method,
                merchant_id,
                transaction_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transactions
            WHERE deleted_at IS NULL
              AND card_number = $1
              AND ($2::TEXT IS NULL
                   OR payment_method ILIKE '%' || $2 || '%'
                   OR status ILIKE '%' || $2 || '%')
            ORDER BY transaction_time DESC
            LIMIT $3 OFFSET $4;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all_by_card_number: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransactionModel {
                    transaction_id: row.try_get("transaction_id")?,
                    card_number: row.try_get("card_number")?,
                    transaction_no: row.try_get("transaction_no")?,
                    amount: row.try_get("amount")?,
                    payment_method: row.try_get("payment_method")?,
                    merchant_id: row.try_get("merchant_id")?,
                    transaction_time: row.try_get("transaction_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transaction by card number rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_id(&self, transaction_id: i32) -> Result<TransactionModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT *
            FROM transactions
            WHERE transaction_id = $1 AND deleted_at IS NULL;
        "#;

        let row = sqlx::query(sql)
            .bind(transaction_id)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!("Transaction not found or database error: {e:?}");
                match e {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound,
                    _ => RepositoryError::Sqlx(e),
                }
            })?;

        let transaction = TransactionModel {
            transaction_id: row.try_get("transaction_id")?,
            card_number: row.try_get("card_number")?,
            transaction_no: row.try_get("transaction_no")?,
            amount: row.try_get("amount")?,
            payment_method: row.try_get("payment_method")?,
            merchant_id: row.try_get("merchant_id")?,
            transaction_time: row.try_get("transaction_time")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        };

        Ok(transaction)
    }

    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<Vec<TransactionModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT *
            FROM transactions
            WHERE merchant_id = $1 AND deleted_at IS NULL
            ORDER BY transaction_time DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(merchant_id)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_merchant_id: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransactionModel {
                    transaction_id: row.try_get("transaction_id")?,
                    card_number: row.try_get("card_number")?,
                    transaction_no: row.try_get("transaction_no")?,
                    amount: row.try_get("amount")?,
                    payment_method: row.try_get("payment_method")?,
                    merchant_id: row.try_get("merchant_id")?,
                    transaction_time: row.try_get("transaction_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transactions by merchant ID: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok(data)
    }
}
