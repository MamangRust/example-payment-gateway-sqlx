use crate::{
    abstract_trait::user::repository::command::UserCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::user::{CreateUserRequest, UpdateUserRequest},
    errors::RepositoryError,
    model::user::UserModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::error;

#[derive(Clone)]
pub struct UserCommandRepository {
    db_pool: ConnectionPool,
}

impl UserCommandRepository {
    pub fn new(db_pool: ConnectionPool) -> Self {
        Self { db_pool }
    }

    async fn get_conn(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, RepositoryError> {
        self.db_pool.acquire().await.map_err(|e| {
            error!("❌ Failed to acquire DB connection: {e:?}");
            RepositoryError::from(e)
        })
    }
}

#[async_trait]
impl UserCommandRepositoryTrait for UserCommandRepository {
    async fn create(&self, req: &CreateUserRequest) -> Result<UserModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (
                firstname,
                lastname,
                email,
                password,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING
                user_id ,
                firstname,
                lastname ,
                email,
                password,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.firstname,
            req.lastname,
            req.email,
            req.password
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in create user: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn update(&self, req: &UpdateUserRequest) -> Result<UserModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET
                firstname = COALESCE($2, firstname),
                lastname = COALESCE($3, lastname),
                email = COALESCE($4, email),
                password = $5,
                updated_at = CURRENT_TIMESTAMP
            WHERE
                user_id = $1
                AND deleted_at IS NULL
            RETURNING
                user_id ,
                firstname,
                lastname ,
                email,
                password,
                created_at,
                updated_at,
                deleted_at
            "#,
            req.id,
            req.firstname,
            req.lastname,
            req.email,
            req.password
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in update user: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn trashed(&self, user_id: i32) -> Result<UserModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET
                deleted_at = CURRENT_TIMESTAMP
            WHERE
                user_id = $1
                AND deleted_at IS NULL
            RETURNING
                user_id,
                firstname ,
                lastname,
                email,
                password,
                created_at,
                updated_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in trashed user: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn restore(&self, user_id: i32) -> Result<UserModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let record = sqlx::query_as!(
            UserModel,
            r#"
            UPDATE users
            SET
                deleted_at = NULL
            WHERE
                user_id = $1
                AND deleted_at IS NOT NULL
            RETURNING
                user_id ,
                firstname,
                lastname,
                email,
                password,
                created_at,
                updated_at,
                deleted_at
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in restore user: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(record)
    }

    async fn delete_permanent(&self, user_id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE user_id = $1 AND deleted_at IS NOT NULL
            "#,
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in delete_permanent user: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            UPDATE users
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in restore_all users: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Database error in delete_all_permanent users: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }
}
