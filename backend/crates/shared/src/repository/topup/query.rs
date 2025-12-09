use crate::{
    abstract_trait::topup::repository::query::TopupQueryRepositoryTrait,
    config::ConnectionPool,
    domain::requests::topup::{FindAllTopups, FindAllTopupsByCardNumber},
    errors::RepositoryError,
    model::topup::TopupModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct TopupQueryRepository {
    db: ConnectionPool,
}

impl TopupQueryRepository {
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
impl TopupQueryRepositoryTrait for TopupQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError> {
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
                t.topup_id,
                t.card_number,
                t.topup_no,
                t.topup_amount,
                t.topup_method,
                t.topup_time,
                t.status,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM topups t
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR 
                   t.card_number ILIKE '%' || $1 || '%' OR 
                   t.topup_no::TEXT ILIKE '%' || $1 || '%' OR 
                   t.topup_method ILIKE '%' || $1 || '%' OR
                   t.status ILIKE '%' || $1 || '%')
            ORDER BY t.topup_time DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("Failed to fetch topups: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let topups = rows
            .into_iter()
            .map(|r| TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((topups, total))
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTopupsByCardNumber,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError> {
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
                t.topup_id,
                t.card_number,
                t.topup_no,
                t.topup_amount,
                t.topup_method,
                t.topup_time,
                t.status,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM topups t
            WHERE t.deleted_at IS NULL
              AND t.card_number = $1
              AND ($2::TEXT IS NULL OR 
                   t.topup_no::TEXT ILIKE '%' || $2 || '%' OR 
                   t.topup_method ILIKE '%' || $2 || '%' OR
                   t.status ILIKE '%' || $2 || '%')
            ORDER BY t.topup_time DESC
            LIMIT $3 OFFSET $4
            "#,
            req.card_number,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!(
                "❌ Failed to fetch topups by card number {}: {e:?}",
                req.card_number,
            );
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let topups = rows
            .into_iter()
            .map(|r| TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((topups, total))
    }

    async fn find_active(
        &self,
        req: &FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError> {
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
                t.topup_id,
                t.card_number,
                t.topup_no,
                t.topup_amount,
                t.topup_method,
                t.topup_time,
                t.status,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM topups t
            WHERE t.deleted_at IS NULL
              AND t.status = 'active'
              AND ($1::TEXT IS NULL OR 
                   t.card_number ILIKE '%' || $1 || '%' OR 
                   t.topup_no::TEXT ILIKE '%' || $1 || '%' OR 
                   t.topup_method ILIKE '%' || $1 || '%')
            ORDER BY t.topup_time DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch active topups: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let topups = rows
            .into_iter()
            .map(|r| TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((topups, total))
    }

    async fn find_trashed(
        &self,
        req: &FindAllTopups,
    ) -> Result<(Vec<TopupModel>, i64), RepositoryError> {
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
                t.topup_id,
                t.card_number,
                t.topup_no,
                t.topup_amount,
                t.topup_method,
                t.topup_time,
                t.status,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM topups t
            WHERE t.deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL OR 
                   t.card_number ILIKE '%' || $1 || '%' OR 
                   t.topup_no::TEXT ILIKE '%' || $1 || '%' OR 
                   t.topup_method ILIKE '%' || $1 || '%')
            ORDER BY t.topup_time DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch trashed topups: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let topups = rows
            .into_iter()
            .map(|r| TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((topups, total))
    }

    async fn find_by_card(&self, card_number: &str) -> Result<Vec<TopupModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let rows = sqlx::query!(
            r#"
        SELECT
            t.topup_id,
            t.card_number,
            t.topup_no,
            t.topup_amount,
            t.topup_method,
            t.topup_time,
            t.status,
            t.created_at,
            t.updated_at,
            t.deleted_at
        FROM topups t
        WHERE t.card_number = $1 AND t.deleted_at IS NULL
        "#,
            card_number
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error when fetching topup card_number {card_number}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        if rows.is_empty() {
            return Err(RepositoryError::Custom(format!(
                "No topups found for card_number {card_number}"
            )));
        }

        let topups = rows
            .into_iter()
            .map(|r| TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok(topups)
    }

    async fn find_by_id(&self, id: i32) -> Result<TopupModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT
                t.topup_id,
                t.card_number,
                t.topup_no,
                t.topup_amount,
                t.topup_method,
                t.topup_time,
                t.status,
                t.created_at,
                t.updated_at,
                t.deleted_at
            FROM topups t
            WHERE t.topup_id = $1 AND t.deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error when fetching topup ID {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(r) => Ok(TopupModel {
                topup_id: r.topup_id,
                card_number: r.card_number,
                topup_no: r.topup_no,
                topup_amount: r.topup_amount as i64,
                topup_method: r.topup_method,
                topup_time: r.topup_time,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            }),
            None => Err(RepositoryError::Custom(format!(
                "Topup with ID {id} not found",
            ))),
        }
    }
}
