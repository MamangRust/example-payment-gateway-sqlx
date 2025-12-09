use crate::{
    abstract_trait::role::repository::query::RoleQueryRepositoryTrait, config::ConnectionPool,
    domain::requests::role::FindAllRoles, errors::RepositoryError, model::role::RoleModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

#[derive(Clone)]
pub struct RoleQueryRepository {
    db: ConnectionPool,
}

impl RoleQueryRepository {
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
impl RoleQueryRepositoryTrait for RoleQueryRepository {
    async fn find_all(&self, req: &FindAllRoles) -> Result<(Vec<RoleModel>, i64), RepositoryError> {
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
            SELECT role_id, role_name, created_at, updated_at, deleted_at, COUNT(*) OVER() AS total_count
            FROM roles
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR role_name ILIKE '%' || $1 || '%')
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch roles: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows
            .first()
            .map(|r| r.total_count.unwrap_or(0))
            .unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| RoleModel {
                role_id: r.role_id,
                role_name: r.role_name,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }

    async fn find_active(
        &self,
        req: &FindAllRoles,
    ) -> Result<(Vec<RoleModel>, i64), RepositoryError> {
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
            SELECT role_id, role_name, created_at, updated_at, deleted_at, COUNT(*) OVER() AS total_count
            FROM roles
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL OR role_name ILIKE '%' || $1 || '%')
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Error fetching active roles: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows
            .first()
            .map(|r| r.total_count.unwrap_or(0))
            .unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| RoleModel {
                role_id: r.role_id,
                role_name: r.role_name,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }

    async fn find_trashed(
        &self,
        req: &FindAllRoles,
    ) -> Result<(Vec<RoleModel>, i64), RepositoryError> {
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
            SELECT role_id, role_name, created_at, updated_at, deleted_at, COUNT(*) OVER() AS total_count
            FROM roles
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL OR role_name ILIKE '%' || $1 || '%')
            ORDER BY deleted_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
             limit as i64,
            offset as i64
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Error fetching trashed roles: {e:?}");
            RepositoryError::from(e)
        })?;

        let total = rows
            .first()
            .map(|r| r.total_count.unwrap_or(0))
            .unwrap_or(0);

        let result = rows
            .into_iter()
            .map(|r| RoleModel {
                role_id: r.role_id,
                role_name: r.role_name,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((result, total))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<RoleModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"SELECT role_id, role_name, created_at, updated_at, deleted_at FROM roles WHERE role_id = $1"#,
            id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Error fetching role by id {id}: {e:?}");
            RepositoryError::from(e)
        })?;

        Ok(result)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<RoleModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"SELECT role_id, role_name, created_at, updated_at, deleted_at FROM roles WHERE role_name = $1"#,
            name
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Error fetching role by name '{name}': {e:?}");
            RepositoryError::from(e)
        })?;

        Ok(result)
    }

    async fn find_by_user_id(&self, user_id: i32) -> Result<Vec<RoleModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let rows = sqlx::query!(
            r#"
            SELECT r.role_id, r.role_name, r.created_at, r.updated_at, r.deleted_at
            FROM roles r
            JOIN user_roles ur ON ur.role_id = r.role_id
            WHERE ur.user_id = $1
            ORDER BY r.created_at ASC
            "#,
            user_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Error fetching roles by user_id {user_id}: {e:?}");
            RepositoryError::from(e)
        })?;

        let result = rows
            .into_iter()
            .map(|r| RoleModel {
                role_id: r.role_id,
                role_name: r.role_name,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok(result)
    }
}
