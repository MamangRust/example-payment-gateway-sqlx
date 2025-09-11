use crate::{
    abstract_trait::user::repository::query::UserQueryRepositoryTrait, config::ConnectionPool,
    domain::requests::user::FindAllUserRequest, errors::RepositoryError, model::user::UserModel,
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::Row;
use tracing::{error, info};

#[derive(Clone)]
pub struct UserQueryRepository {
    db: ConnectionPool,
}

impl UserQueryRepository {
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
impl UserQueryRepositoryTrait for UserQueryRepository {
    async fn find_all(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError> {
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
                *,
                COUNT(*) OVER() AS total_count
            FROM users
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL 
                   OR firstname ILIKE '%' || $1 || '%' 
                   OR lastname ILIKE '%' || $1 || '%' 
                   OR email ILIKE '%' || $1 || '%')
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_all users: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(UserModel {
                    user_id: row.try_get("user_id")?,
                    firstname: row.try_get("firstname")?,
                    lastname: row.try_get("lastname")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map user rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_active(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError> {
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
                *,
                COUNT(*) OVER() AS total_count
            FROM users
            WHERE deleted_at IS NULL
              AND ($1::TEXT IS NULL 
                   OR firstname ILIKE '%' || $1 || '%' 
                   OR lastname ILIKE '%' || $1 || '%' 
                   OR email ILIKE '%' || $1 || '%')
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_active users: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(UserModel {
                    user_id: row.try_get("user_id")?,
                    firstname: row.try_get("firstname")?,
                    lastname: row.try_get("lastname")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map user rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_trashed(
        &self,
        req: FindAllUserRequest,
    ) -> Result<(Vec<UserModel>, i64), RepositoryError> {
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
                *,
                COUNT(*) OVER() AS total_count
            FROM users
            WHERE deleted_at IS NOT NULL
              AND ($1::TEXT IS NULL 
                   OR firstname ILIKE '%' || $1 || '%' 
                   OR lastname ILIKE '%' || $1 || '%' 
                   OR email ILIKE '%' || $1 || '%')
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3;
        "#;

        let rows = sqlx::query(sql)
            .bind(search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in find_by_trashed users: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let total = rows
            .first()
            .and_then(|r| r.try_get::<i64, _>("total_count").ok())
            .unwrap_or(0);

        let data = rows
            .into_iter()
            .map(|row| {
                Ok(UserModel {
                    user_id: row.try_get("user_id")?,
                    firstname: row.try_get("firstname")?,
                    lastname: row.try_get("lastname")?,
                    email: row.try_get("email")?,
                    password: row.try_get("password")?,
                    created_at: row.try_get("created_at")?,
                    updated_at: row.try_get("updated_at")?,
                    deleted_at: row.try_get("deleted_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| {
                error!("Failed to map trashed user rows: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        Ok((data, total))
    }

    async fn find_by_id(&self, user_id: i32) -> Result<UserModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT * FROM users 
            WHERE user_id = $1 AND deleted_at IS NULL;
        "#;

        let row = sqlx::query(sql)
            .bind(user_id)
            .fetch_one(&mut *conn)
            .await
            .map_err(|e| {
                error!("User not found or database error: {e:?}");
                match e {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound,
                    _ => RepositoryError::Sqlx(e),
                }
            })?;

        let user = UserModel {
            user_id: row.try_get("user_id")?,
            firstname: row.try_get("firstname")?,
            lastname: row.try_get("lastname")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        };

        Ok(user)
    }

    async fn find_by_email(&self, email: String) -> Result<Option<UserModel>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT * FROM users 
            WHERE email = $1 AND deleted_at IS NULL;
        "#;

        let row = match sqlx::query(sql).bind(&email).fetch_one(&mut *conn).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => {
                info!("📭 No user found with email: {email}");
                return Ok(None);
            }
            Err(e) => {
                error!("🗄️ Database error while querying user by email '{email}': {e:?}");
                return Err(RepositoryError::Sqlx(e));
            }
        };

        let user = UserModel {
            user_id: row.try_get("user_id")?,
            firstname: row.try_get("firstname")?,
            lastname: row.try_get("lastname")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        };

        Ok(Some(user))
    }
}
