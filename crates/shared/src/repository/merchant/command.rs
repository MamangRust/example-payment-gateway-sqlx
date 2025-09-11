use crate::{
    abstract_trait::merchant::repository::command::MerchantCommandRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::{
        CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus,
    },
    errors::RepositoryError,
    model::merchant::MerchantModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantCommandRepository {
    db: ConnectionPool,
}

impl MerchantCommandRepository {
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
impl MerchantCommandRepositoryTrait for MerchantCommandRepository {
    async fn create(
        &self,
        api_key: String,
        request: &CreateMerchantRequest,
    ) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let merchant = sqlx::query_as!(
            MerchantModel,
            r#"
            INSERT INTO merchants (
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING
                merchant_id,
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            request.name,
            api_key,
            request.user_id,
            "inactive".to_string(),
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to create merchant: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(merchant)
    }

    async fn update(
        &self,
        request: &UpdateMerchantRequest,
    ) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let merchant = sqlx::query_as!(
            MerchantModel,
            r#"
            UPDATE merchants
            SET
                name = COALESCE($2, name),
                user_id = COALESCE($3, user_id),
                status = COALESCE($4, status),
                updated_at = NOW()
            WHERE merchant_id = $1 AND deleted_at IS NULL
            RETURNING
                merchant_id,
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            request.merchant_id,
            request.name,
            request.user_id,
            request.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!(
                    "❌ Merchant not found or already deleted: {}",
                    request.merchant_id
                );
                RepositoryError::NotFound
            }
            _ => {
                error!(
                    "❌ Failed to update merchant {}: {e:?}",
                    request.merchant_id,
                );
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(merchant)
    }

    async fn update_status(
        &self,
        request: &UpdateMerchantStatus,
    ) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        info!(
            "🔄 Updating status for merchant ID: {}",
            request.merchant_id
        );

        let merchant = sqlx::query_as!(
            MerchantModel,
            r#"
            UPDATE merchants
            SET status = $2, updated_at = NOW()
            WHERE merchant_id = $1 AND deleted_at IS NULL
            RETURNING
                merchant_id,
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            request.merchant_id,
            request.status
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!(
                    "❌ Merchant not found or already deleted: {}",
                    request.merchant_id
                );
                RepositoryError::NotFound
            }
            _ => {
                error!(
                    "❌ Failed to update status for merchant {}: {e:?}",
                    request.merchant_id,
                );
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(merchant)
    }

    async fn trash(&self, id: i32) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let merchant = sqlx::query_as!(
            MerchantModel,
            r#"
            UPDATE merchants
            SET deleted_at = NOW()
            WHERE merchant_id = $1 AND deleted_at IS NULL
            RETURNING
                merchant_id,
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Merchant not found or already trashed: {id}");
                RepositoryError::NotFound
            }
            _ => {
                error!("❌ Failed to trash merchant {id}: {e:?}");
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(merchant)
    }

    async fn restore(&self, id: i32) -> Result<MerchantModel, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let merchant = sqlx::query_as!(
            MerchantModel,
            r#"
            UPDATE merchants
            SET deleted_at = NULL
            WHERE merchant_id = $1 AND deleted_at IS NOT NULL
            RETURNING
                merchant_id,
                name,
                api_key,
                user_id,
                status,
                created_at,
                updated_at,
                deleted_at
            "#,
            id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                error!("❌ Merchant not found or not trashed: {id}");
                RepositoryError::NotFound
            }
            _ => {
                error!("❌ Failed to restore merchant {id}: {e:?}");
                RepositoryError::Sqlx(e)
            }
        })?;

        Ok(merchant)
    }

    async fn delete_permanent(&self, id: i32) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
                DELETE FROM merchants
                WHERE merchant_id = $1 AND deleted_at IS NOT NULL
            "#,
            id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to permanently delete merchant {id}: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            UPDATE merchants
            SET deleted_at = NULL
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to restore all trashed merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_all(&self) -> Result<bool, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM merchants
            WHERE deleted_at IS NOT NULL
            "#
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to delete all trashed merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        Ok(result.rows_affected() > 0)
    }
}
