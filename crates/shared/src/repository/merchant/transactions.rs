use crate::{
    abstract_trait::merchant::repository::transactions::MerchantTransactionRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::{
        FindAllMerchantTransactions, FindAllMerchantTransactionsByApiKey,
        FindAllMerchantTransactionsById,
    },
    errors::RepositoryError,
    model::merchant::MerchantTransactionsModel,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct MerchantTransactionRepository {
    db: ConnectionPool,
}

impl MerchantTransactionRepository {
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
impl MerchantTransactionRepositoryTrait for MerchantTransactionRepository {
    async fn find_all_transactiions(
        &self,
        req: &FindAllMerchantTransactions,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError> {
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
                t.transaction_id,
                t.card_number,
                t.amount,
                t.payment_method,
                t.merchant_id,
                m.name AS merchant_name,
                t.transaction_time,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                transactions t
            JOIN
                merchants m ON t.merchant_id = m.merchant_id
            WHERE
                t.deleted_at IS NULL
                AND ($1::TEXT IS NULL OR t.card_number ILIKE '%' || $1 || '%' OR t.payment_method ILIKE '%' || $1 || '%')
            ORDER BY
                t.transaction_time DESC
            LIMIT $2 OFFSET $3
            "#,
            search_pattern,
            limit as i64,
            offset as i64
        ).fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch all merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantTransactionsModel {
                transaction_id: r.transaction_id,
                card_number: r.card_number,
                merchant_id: r.merchant_id,
                merchant_name: r.merchant_name,
                amount: r.amount,
                payment_method: r.payment_method,
                transaction_time: r.transaction_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }

    async fn find_all_transactiions_by_api_key(
        &self,
        req: &FindAllMerchantTransactionsByApiKey,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError> {
        let mut conn = self.get_conn().await?;

        info!(
            "🔍 Fetching transactions for api_key: {} with search: {:?}",
            req.api_key, req.search
        );

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
                t.transaction_id,
                t.card_number,
                t.amount,
                t.payment_method,
                t.merchant_id,
                m.name AS merchant_name,
                t.transaction_time,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                transactions t
            JOIN
                merchants m ON t.merchant_id = m.merchant_id
            WHERE
                t.deleted_at IS NULL
                AND m.api_key = $1
                AND ($2::TEXT IS NULL OR t.card_number ILIKE '%' || $2 || '%' OR t.payment_method ILIKE '%' || $2 || '%')
            ORDER BY
                t.transaction_time DESC
            LIMIT $3 OFFSET $4
            "#,
            req.api_key,
            search_pattern,
            limit as i64,
            offset as i64,
        ).fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch all merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantTransactionsModel {
                transaction_id: r.transaction_id,
                card_number: r.card_number,
                merchant_id: r.merchant_id,
                merchant_name: r.merchant_name,
                amount: r.amount,
                payment_method: r.payment_method,
                transaction_time: r.transaction_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }
    async fn find_all_transactiions_by_id(
        &self,
        req: &FindAllMerchantTransactionsById,
    ) -> Result<(Vec<MerchantTransactionsModel>, i64), RepositoryError> {
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
                t.transaction_id,
                t.card_number,
                t.amount,
                t.payment_method,
                t.merchant_id,
                m.name AS merchant_name,
                t.transaction_time,
                t.created_at,
                t.updated_at,
                t.deleted_at,
                COUNT(*) OVER() AS total_count
            FROM
                transactions t
            JOIN
                merchants m ON t.merchant_id = m.merchant_id
            WHERE
                t.deleted_at IS NULL
                AND t.merchant_id = $1
                AND ($2::TEXT IS NULL OR t.card_number ILIKE '%' || $2 || '%' OR t.payment_method ILIKE '%' || $2 || '%')
            ORDER BY
                t.transaction_time DESC
            LIMIT $3 OFFSET $4
            "#,
            req.merchant_id,
            search_pattern,
            limit as i64,
            offset as i64,
        ).fetch_all(&mut *conn)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch all merchants: {e:?}");
            RepositoryError::Sqlx(e)
        })?;

        let total = rows.first().and_then(|r| r.total_count).unwrap_or(0);

        let merchants = rows
            .into_iter()
            .map(|r| MerchantTransactionsModel {
                transaction_id: r.transaction_id,
                card_number: r.card_number,
                merchant_id: r.merchant_id,
                merchant_name: r.merchant_name,
                amount: r.amount,
                payment_method: r.payment_method,
                transaction_time: r.transaction_time,
                created_at: r.created_at,
                updated_at: r.updated_at,
                deleted_at: r.deleted_at,
            })
            .collect();

        Ok((merchants, total))
    }
}
