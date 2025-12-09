use crate::{
    abstract_trait::withdraw::repository::query::WithdrawQueryRepositoryTrait,
    config::ConnectionPool,
    domain::requests::withdraw::{FindAllWithdrawCardNumber, FindAllWithdraws},
    errors::RepositoryError,
    model::withdraw::WithdrawModel,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use tracing::error;

pub struct WithdrawQueryRepository {
    db: ConnectionPool,
}

impl WithdrawQueryRepository {
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
impl WithdrawQueryRepositoryTrait for WithdrawQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError> {
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
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount,
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                withdraws
            WHERE
                deleted_at IS NULL
                AND ($1::TEXT IS NULL
                    OR card_number ILIKE '%' || $1 || '%'
                    OR withdraw_amount::TEXT ILIKE '%' || $1 || '%'
                    OR withdraw_time::TEXT ILIKE '%' || $1 || '%'
                    OR status ILIKE '%' || $1 || '%'
                )
            ORDER BY
                withdraw_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all withdraws: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(WithdrawModel {
                    withdraw_id: row.try_get("withdraw_id")?,
                    withdraw_no: row.try_get("withdraw_no")?,
                    card_number: row.try_get("card_number")?,
                    withdraw_amount: row.try_get("withdraw_amount")?,
                    withdraw_time: row.try_get("withdraw_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map withdraw rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_active(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError> {
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
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount,
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                withdraws
            WHERE
                deleted_at IS NULL
                AND ($1::TEXT IS NULL
                    OR card_number ILIKE '%' || $1 || '%'
                    OR withdraw_amount::TEXT ILIKE '%' || $1 || '%'
                    OR withdraw_time::TEXT ILIKE '%' || $1 || '%'
                    OR status ILIKE '%' || $1 || '%'
                )
            ORDER BY
                withdraw_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all withdraws: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(WithdrawModel {
                    withdraw_id: row.try_get("withdraw_id")?,
                    withdraw_no: row.try_get("withdraw_no")?,
                    card_number: row.try_get("card_number")?,
                    withdraw_amount: row.try_get("withdraw_amount")?,
                    withdraw_time: row.try_get("withdraw_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map withdraw rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllWithdraws,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError> {
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
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount,
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                withdraws
            WHERE
                deleted_at IS NOT NULL
                AND ($1::TEXT IS NULL
                    OR card_number ILIKE '%' || $1 || '%'
                    OR withdraw_amount::TEXT ILIKE '%' || $1 || '%'
                    OR withdraw_time::TEXT ILIKE '%' || $1 || '%'
                    OR status ILIKE '%' || $1 || '%'
                )
            ORDER BY
                withdraw_time DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_trashed withdraws: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(WithdrawModel {
                    withdraw_id: row.try_get("withdraw_id")?,
                    withdraw_no: row.try_get("withdraw_no")?,
                    card_number: row.try_get("card_number")?,
                    withdraw_amount: row.try_get("withdraw_amount")?,
                    withdraw_time: row.try_get("withdraw_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map withdraw rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllWithdrawCardNumber,
    ) -> Result<(Vec<WithdrawModel>, i64), RepositoryError> {
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
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount,
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                withdraws
            WHERE
                deleted_at IS NULL
                AND card_number = $1
                AND (
                    $2::TEXT IS NULL
                    OR withdraw_amount::TEXT ILIKE '%' || $2 || '%'
                    OR withdraw_time::TEXT ILIKE '%' || $2 || '%'
                    OR status ILIKE '%' || $2 || '%'
                )
            ORDER BY
                withdraw_time DESC
            LIMIT $3 OFFSET $4;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
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
                Ok(WithdrawModel {
                    withdraw_id: row.try_get("withdraw_id")?,
                    withdraw_no: row.try_get("withdraw_no")?,
                    card_number: row.try_get("card_number")?,
                    withdraw_amount: row.try_get("withdraw_amount")?,
                    withdraw_time: row.try_get("withdraw_time")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map withdraw rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<WithdrawModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                withdraw_id,
                withdraw_no,
                card_number,
                withdraw_amount,
                withdraw_time,
                status,
                created_at,
                updated_at,
                deleted_at
            FROM
                withdraws
            WHERE
                withdraw_id = $1
                AND deleted_at IS NULL;
        "#;

        let row = sqlx::query(sql)
            .bind(id)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_id: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let model = WithdrawModel {
            withdraw_id: row.try_get("withdraw_id")?,
            withdraw_no: row.try_get("withdraw_no")?,
            card_number: row.try_get("card_number")?,
            withdraw_amount: row.try_get("withdraw_amount")?,
            withdraw_time: row.try_get("withdraw_time")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        };

        Ok(model)
    }

    async fn find_by_card(&self, card_number: &str) -> Result<Vec<WithdrawModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
        SELECT
            withdraw_id,
            withdraw_no,
            card_number,
            withdraw_amount,
            withdraw_time,
            status,
            created_at,
            updated_at,
            deleted_at
        FROM
            withdraws
        WHERE
            card_number = $1
            AND deleted_at IS NULL;
    "#;

        let rows = sqlx::query(sql)
            .bind(card_number)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_card for card_number={card_number}: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        if rows.is_empty() {
            return Err(RepositoryError::Custom(format!(
                "No withdrawals found for card_number {card_number}"
            )));
        }

        let withdraws = rows
            .into_iter()
            .map(|row| WithdrawModel {
                withdraw_id: row.try_get("withdraw_id").unwrap(),
                withdraw_no: row.try_get("withdraw_no").unwrap(),
                card_number: row.try_get("card_number").unwrap(),
                withdraw_amount: row.try_get("withdraw_amount").unwrap(),
                withdraw_time: row.try_get("withdraw_time").unwrap(),
                status: row.try_get("status").unwrap(),
                created_at: row.try_get("created_at").unwrap(),
                updated_at: row.try_get("updated_at").unwrap(),
                deleted_at: row.try_get("deleted_at").unwrap(),
            })
            .collect();

        Ok(withdraws)
    }
}
