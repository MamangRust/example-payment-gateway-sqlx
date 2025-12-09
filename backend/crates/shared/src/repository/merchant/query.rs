use crate::{
    abstract_trait::merchant::repository::query::MerchantQueryRepositoryTrait,
    config::ConnectionPool, domain::requests::merchant::FindAllMerchants, errors::RepositoryError,
    model::merchant::MerchantModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantQueryRepository {
    db: ConnectionPool,
}

impl MerchantQueryRepository {
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
impl MerchantQueryRepositoryTrait for MerchantQueryRepository {
    async fn find_all(
        &self,
        req: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError> {
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
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM merchants m
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR 
                   m.name ILIKE '%' || $1 || '%' OR 
                   m.api_key ILIKE '%' || $1 || '%' OR 
                   m.status ILIKE '%' || $1 || '%')
            ORDER BY m.merchant_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch all merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantModel {
                merchant_id: r.merchant_id,
                name: r.name,
                api_key: r.api_key,
                user_id: r.user_id,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }

    async fn find_active(
        &self,
        req: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError> {
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
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM merchants m
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR 
                   m.name ILIKE '%' || $1 || '%' OR 
                   m.api_key ILIKE '%' || $1 || '%' OR 
                   m.status ILIKE '%' || $1 || '%')
            ORDER BY m.merchant_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch active merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantModel {
                merchant_id: r.merchant_id,
                name: r.name,
                api_key: r.api_key,
                user_id: r.user_id,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }

    async fn find_trashed(
        &self,
        req: &FindAllMerchants,
    ) -> Result<(Vec<MerchantModel>, i64), RepositoryError> {
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
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM merchants m
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL OR 
                   m.name ILIKE '%' || $1 || '%' OR 
                   m.api_key ILIKE '%' || $1 || '%' OR 
                   m.status ILIKE '%' || $1 || '%')
            ORDER BY m.merchant_id
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch trashed merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantModel {
                merchant_id: r.merchant_id,
                name: r.name,
                api_key: r.api_key,
                user_id: r.user_id,
                status: r.status,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        info!("🔍 Fetching merchant by ID: {}", id);

        let row = sqlx::query_as!(
            MerchantModel,
            r#"
            SELECT
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at
            FROM merchants m
            WHERE m.merchant_id = $1 AND m.deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch merchant by ID {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(merchant) => Ok(merchant),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_by_apikey(&self, api_key: &str) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        info!("🔍 Fetching merchant by API key");

        let row = sqlx::query_as!(
            MerchantModel,
            r#"
            SELECT
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at
            FROM merchants m
            WHERE m.api_key = $1 AND m.deleted_at IS NULL
            "#,
            api_key
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch merchant by API key: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(merchant) => Ok(merchant),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        info!("🔍 Fetching merchant by name: {}", name);

        let row = sqlx::query_as!(
            MerchantModel,
            r#"
            SELECT
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at
            FROM merchants m
            WHERE m.name = $1 AND m.deleted_at IS NULL
            "#,
            name
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch merchant by name {name}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        match row {
            Some(merchant) => Ok(merchant),
            None => Err(RepositoryError::NotFound),
        }
    }

    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<Vec<MerchantModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let rows = sqlx::query_as!(
            MerchantModel,
            r#"
            SELECT
                m.merchant_id,
                m.name,
                m.api_key,
                m.user_id,
                m.status,
                m.created_at,
                m.updated_at,
                m.deleted_at
            FROM merchants m
            WHERE m.user_id = $1 AND m.deleted_at IS NULL
            ORDER BY m.merchant_id
            "#,
            user_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch merchants by user_id {user_id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(rows)
    }
}
