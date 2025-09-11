use crate::{
    abstract_trait::transaction::repository::stats::amount::TransactionStatsAmountRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::transaction::{TransactionMonthAmount, TransactionYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct TransactionStatsAmountRepository {
    db: ConnectionPool,
}

impl TransactionStatsAmountRepository {
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
impl TransactionStatsAmountRepositoryTrait for TransactionStatsAmountRepository {
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionMonthAmount>, RepositoryError> {
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
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS month,
                COALESCE(SUM(t.amount), 0)::bigint AS total_amount
            FROM
                months m
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_amounts: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
        for row in rows {
            let month: String = row.try_get("month")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionMonthAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<Vec<TransactionYearlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM t.transaction_time)::text AS year,
                SUM(t.amount)::bigint AS total_amount
            FROM
                transactions t
            WHERE
                t.deleted_at IS NULL
                AND EXTRACT(YEAR FROM t.transaction_time) >= $1 - 4
                AND EXTRACT(YEAR FROM t.transaction_time) <= $1
            GROUP BY
                EXTRACT(YEAR FROM t.transaction_time)
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_amounts: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionYearlyAmount { year, total_amount });
        }

        Ok(result)
    }
}
