use crate::{
    abstract_trait::transfer::repository::query::TransferQueryRepositoryTrait,
    config::ConnectionPool, domain::requests::transfer::FindAllTransfers, errors::RepositoryError,
    model::transfer::TransferModel,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use tracing::error;

pub struct TransferQueryRepository {
    db: ConnectionPool,
}

impl TransferQueryRepository {
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
impl TransferQueryRepositoryTrait for TransferQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError> {
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
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transfers
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL
                   OR transfer_from ILIKE '%' || $1 || '%'
                   OR transfer_to ILIKE '%' || $1 || '%')
            ORDER BY transfer_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all transfers: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransferModel {
                    transfer_id: row.try_get("transfer_id")?,
                    transfer_no: row.try_get("transfer_no")?,
                    transfer_from: row.try_get("transfer_from")?,
                    transfer_to: row.try_get("transfer_to")?,
                    transfer_amount: row.try_get("transfer_amount")?,
                    transfer_time: row.try_get("transfer_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transfer rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError> {
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
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transfers
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL
                   OR transfer_from ILIKE '%' || $1 || '%'
                   OR transfer_to ILIKE '%' || $1 || '%')
            ORDER BY transfer_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_active transfers: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransferModel {
                    transfer_id: row.try_get("transfer_id")?,
                    transfer_no: row.try_get("transfer_no")?,
                    transfer_from: row.try_get("transfer_from")?,
                    transfer_to: row.try_get("transfer_to")?,
                    transfer_amount: row.try_get("transfer_amount")?,
                    transfer_time: row.try_get("transfer_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transfer rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllTransfers,
    ) -> Result<(Vec<TransferModel>, i64), RepositoryError> {
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
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM transfers
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL
                   OR transfer_from ILIKE '%' || $1 || '%'
                   OR transfer_to ILIKE '%' || $1 || '%')
            ORDER BY transfer_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_trashed transfers: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransferModel {
                    transfer_id: row.try_get("transfer_id")?,
                    transfer_no: row.try_get("transfer_no")?,
                    transfer_from: row.try_get("transfer_from")?,
                    transfer_to: row.try_get("transfer_to")?,
                    transfer_amount: row.try_get("transfer_amount")?,
                    transfer_time: row.try_get("transfer_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map trashed transfer rows: {e:?}",);
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<TransferModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            FROM transfers
            WHERE transfer_id = $1 AND deleted_at IS NULL;
        "#;

        let row = sqlx::query(sql)
            .bind(id)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_id transfer: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let model = TransferModel {
            transfer_id: row.try_get("transfer_id")?,
            transfer_no: row.try_get("transfer_no")?,
            transfer_from: row.try_get("transfer_from")?,
            transfer_to: row.try_get("transfer_to")?,
            transfer_amount: row.try_get("transfer_amount")?,
            transfer_time: row.try_get("transfer_time")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        };

        Ok(model)
    }

    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<Vec<TransferModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            FROM transfers
            WHERE deleted_at IS NULL AND transfer_from = $1
            ORDER BY transfer_time DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(transfer_from)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_transfer_from: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransferModel {
                    transfer_id: row.try_get("transfer_id")?,
                    transfer_no: row.try_get("transfer_no")?,
                    transfer_from: row.try_get("transfer_from")?,
                    transfer_to: row.try_get("transfer_to")?,
                    transfer_amount: row.try_get("transfer_amount")?,
                    transfer_time: row.try_get("transfer_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transfers by source: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok(data)
    }

    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<Vec<TransferModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                transfer_id,
                transfer_no,
                transfer_from,
                transfer_to,
                transfer_amount,
                transfer_time AS transfer_time,
                status,
                created_at,
                updated_at,
                deleted_at
            FROM transfers
            WHERE deleted_at IS NULL AND transfer_to = $1
            ORDER BY transfer_time DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(transfer_to)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_transfer_to: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(TransferModel {
                    transfer_id: row.try_get("transfer_id")?,
                    transfer_no: row.try_get("transfer_no")?,
                    transfer_from: row.try_get("transfer_from")?,
                    transfer_to: row.try_get("transfer_to")?,
                    transfer_amount: row.try_get("transfer_amount")?,
                    transfer_time: row.try_get("transfer_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map transfers by destination: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok(data)
    }
}
