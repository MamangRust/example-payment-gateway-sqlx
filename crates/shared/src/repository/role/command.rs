use crate::{
    abstract_trait::role::repository::command::RoleCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::role::{CreateRoleRequest, UpdateRoleRequest},
    errors::RepositoryError,
    model::role::RoleModel,
};
use async_trait::async_trait;
use tracing::{error, info};

pub struct RoleCommandRepository {
    db: ConnectionPool,
}

impl RoleCommandRepository {
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
impl RoleCommandRepositoryTrait for RoleCommandRepository {
    async fn create(&self, role: &CreateRoleRequest) -> Result<RoleModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"
            INSERT INTO roles (role_name, created_at, updated_at)
            VALUES ($1, current_timestamp, current_timestamp)
            RETURNING role_id, role_name, created_at, updated_at, deleted_at
            "#,
            role.name
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to create role '{}': {err:?}", role.name);
            RepositoryError::from(err)
        })?;

        Ok(result)
    }

    async fn update(&self, role: &UpdateRoleRequest) -> Result<RoleModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"
            UPDATE roles
            SET role_name = $2, updated_at = current_timestamp
            WHERE role_id = $1
            RETURNING role_id, role_name, created_at, updated_at, deleted_at
            "#,
            role.id,
            role.name
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to update role ID {}: {err:?}", role.name);
            RepositoryError::from(err)
        })?;

        Ok(result)
    }

    async fn trash(&self, role_id: i32) -> Result<RoleModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"
            UPDATE roles
            SET deleted_at = current_timestamp
            WHERE role_id = $1
            RETURNING role_id, role_name, created_at, updated_at, deleted_at
            "#,
            role_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to trash role ID {role_id}: {err:?}");
            RepositoryError::from(err)
        })?;

        Ok(result)
    }

    async fn restore(&self, role_id: i32) -> Result<RoleModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query_as!(
            RoleModel,
            r#"
            UPDATE roles
            SET deleted_at = NULL
            WHERE role_id = $1
            RETURNING role_id, role_name, created_at, updated_at, deleted_at
            "#,
            role_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to restore role ID {role_id}: {err:?}");
            RepositoryError::from(err)
        })?;

        Ok(result)
    }

    async fn delete_permanent(&self, role_id: i32) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE role_id = $1 AND deleted_at IS NOT NULL
            "#,
            role_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to permanently delete role ID {role_id}: {err:?}",);
            RepositoryError::from(err)
        })?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn restore_all(&self) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            UPDATE roles
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to restore all roles: {err:?}");
            RepositoryError::from(err)
        })?;

        Ok(())
    }

    async fn delete_all(&self) -> Result<(), RepositoryError> {
        let mut conn = self.get_conn().await?;

        sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            error!("❌ Failed to delete all trashed roles: {err:?}");
            RepositoryError::from(err)
        })?;

        Ok(())
    }
}
