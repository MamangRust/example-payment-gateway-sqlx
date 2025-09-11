use crate::{
    abstract_trait::transaction::repository::stats::method::TransactionStatsMethodRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::transaction::{TransactionMonthMethod, TransactionYearMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct TransactionStatsMethodRepository {
    db: ConnectionPool,
}

impl TransactionStatsMethodRepository {
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
impl TransactionStatsMethodRepositoryTrait for TransactionStatsMethodRepository {
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionMonthMethod>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".to_string()))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let sql = r#"
            WITH months AS (
                SELECT generate_series(
                    date_trunc('year', $1::timestamp),
                    date_trunc('year', $1::timestamp) + interval '1 year' - interval '1 day',
                    interval '1 month'
                ) AS month
            ),
            payment_methods AS (
                SELECT DISTINCT payment_method
                FROM transactions
                WHERE deleted_at IS NULL
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS month,
                pm.payment_method,
                COALESCE(COUNT(t.transaction_id), 0)::int AS total_transactions,
                COALESCE(SUM(t.amount), 0)::bigint AS total_amount
            FROM
                months m
            CROSS JOIN
                payment_methods pm
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.payment_method = pm.payment_method
                AND t.deleted_at IS NULL
            GROUP BY
                m.month,
                pm.payment_method
            ORDER BY
                m.month,
                pm.payment_method;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_method: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let month: String = row.try_get("month")?;
            let payment_method: String = row.try_get("payment_method")?;
            let total_transactions: i32 = row.try_get("total_transactions")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionMonthMethod {
                month,
                payment_method,
                total_transactions,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionYearMethod>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM t.transaction_time)::text AS year,
                t.payment_method,
                COUNT(t.transaction_id)::int AS total_transactions,
                SUM(t.amount)::bigint AS total_amount
            FROM
                transactions t
            WHERE
                t.deleted_at IS NULL
                AND EXTRACT(YEAR FROM t.transaction_time) >= $1 - 4
                AND EXTRACT(YEAR FROM t.transaction_time) <= $1
            GROUP BY
                EXTRACT(YEAR FROM t.transaction_time),
                t.payment_method
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_method: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let payment_method: String = row.try_get("payment_method")?;
            let total_transactions: i32 = row.try_get("total_transactions")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionYearMethod {
                year,
                payment_method,
                total_transactions,
                total_amount,
            });
        }

        Ok(result)
    }
}
