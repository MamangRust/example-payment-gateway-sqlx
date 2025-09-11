use crate::{
    abstract_trait::card::repository::query::CardQueryRepositoryTrait, config::ConnectionPool,
    domain::requests::card::FindAllCards, errors::RepositoryError, model::card::CardModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

pub struct CardQueryRepository {
    db: ConnectionPool,
}

impl CardQueryRepository {
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
impl CardQueryRepositoryTrait for CardQueryRepository {
    async fn find_all(&self, req: &FindAllCards) -> Result<(Vec<CardModel>, i64), RepositoryError> {
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
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM cards c
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR 
                   c.card_number ILIKE '%' || $1 || '%' OR 
                   c.card_type ILIKE '%' || $1 || '%' OR 
                   c.card_provider ILIKE '%' || $1 || '%')
            ORDER BY c.card_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch cards: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let cards = rows
            .into_iter()
            .map(|r| CardModel {
                card_id: r.card_id,
                user_id: r.user_id,
                card_number: r.card_number,
                card_type: r.card_type,
                expire_date: r.expire_date,
                cvv: r.cvv,
                card_provider: r.card_provider,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((cards, total))
    }

    async fn find_active(
        &self,
        req: &FindAllCards,
    ) -> Result<(Vec<CardModel>, i64), RepositoryError> {
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
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM cards c
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR 
                   c.card_number ILIKE '%' || $1 || '%' OR 
                   c.card_type ILIKE '%' || $1 || '%' OR 
                   c.card_provider ILIKE '%' || $1 || '%')
            ORDER BY c.card_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch active cards: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let cards = rows
            .into_iter()
            .map(|r| CardModel {
                card_id: r.card_id,
                user_id: r.user_id,
                card_number: r.card_number,
                card_type: r.card_type,
                expire_date: r.expire_date,
                cvv: r.cvv,
                card_provider: r.card_provider,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((cards, total))
    }

    async fn find_trashed(
        &self,
        req: &FindAllCards,
    ) -> Result<(Vec<CardModel>, i64), RepositoryError> {
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
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM cards c
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL OR 
                   c.card_number ILIKE '%' || $1 || '%' OR 
                   c.card_type ILIKE '%' || $1 || '%' OR 
                   c.card_provider ILIKE '%' || $1 || '%')
            ORDER BY c.card_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch trashed cards: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let cards = rows
            .into_iter()
            .map(|r| CardModel {
                card_id: r.card_id,
                user_id: r.user_id,
                card_number: r.card_number,
                card_type: r.card_type,
                expire_date: r.expire_date,
                cvv: r.cvv,
                card_provider: r.card_provider,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((cards, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at
            FROM cards c
            WHERE c.card_id = $1
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch card by ID {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(r) => Ok(CardModel {
                card_id: r.card_id,
                user_id: r.user_id,
                card_number: r.card_number,
                card_type: r.card_type,
                expire_date: r.expire_date,
                cvv: r.cvv,
                card_provider: r.card_provider,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            }),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_by_card(&self, card_number: &str) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at
            FROM cards c
            WHERE c.card_number = $1 AND c.deleted_at IS NULL
            "#,
            card_number
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch card by number {card_number}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(r) => Ok(CardModel {
                card_id: r.card_id,
                user_id: r.user_id,
                card_number: r.card_number,
                card_type: r.card_type,
                expire_date: r.expire_date,
                cvv: r.cvv,
                card_provider: r.card_provider,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            }),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<CardModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let row = sqlx::query!(
            r#"
            SELECT
                c.card_id,
                c.user_id,
                c.card_number,
                c.card_type,
                c.expire_date,
                c.cvv,
                c.card_provider,
                c.created_at,
                c.updated_at,
                c.deleted_at
            FROM cards c
            WHERE c.user_id = $1 AND c.deleted_at IS NULL
            ORDER BY c.card_id
            LIMIT 1
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch card by user_id {user_id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(CardModel {
            card_id: row.card_id,
            user_id: row.user_id,
            card_number: row.card_number,
            card_type: row.card_type,
            expire_date: row.expire_date,
            cvv: row.cvv,
            card_provider: row.card_provider,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        })
    }
}
