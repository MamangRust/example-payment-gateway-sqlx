use crate::{
    abstract_trait::merchant::repository::stats::totalamount::MerchantStatsTotalAmountRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct MerchantStatsTotalAmountRepository {
    db: ConnectionPool,
}

impl MerchantStatsTotalAmountRepository {
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
impl MerchantStatsTotalAmountRepositoryTrait for MerchantStatsTotalAmountRepository {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("❌ Invalid year".to_string()))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let sql = r#"
            WITH monthly_data AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time)::text AS year,
                    TO_CHAR(t.transaction_time, 'Mon') AS month,
                    COALESCE(SUM(t.amount), 0)::bigint AS total_amount
                FROM
                    transactions t
                INNER JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND (
                        t.transaction_time >= date_trunc('year', $1::timestamp)
                        AND t.transaction_time < date_trunc('year', $1::timestamp) + interval '1 year'
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time),
                    TO_CHAR(t.transaction_time, 'Mon')
            ), missing_months AS (
                SELECT
                    EXTRACT(YEAR FROM $1::timestamp)::text AS year,
                    TO_CHAR($1::timestamp, 'Mon') AS month,
                    0::bigint AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $1::timestamp)::text
                      AND month = TO_CHAR($1::timestamp, 'Mon')
                )
                UNION ALL
                SELECT
                    EXTRACT(YEAR FROM date_trunc('month', $1::timestamp) - interval '1 month')::text AS year,
                    TO_CHAR(date_trunc('month', $1::timestamp) - interval '1 month', 'Mon') AS month,
                    0::bigint AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM date_trunc('month', $1::timestamp) - interval '1 month')::text
                      AND month = TO_CHAR(date_trunc('month', $1::timestamp) - interval '1 month', 'Mon')
                )
            )
            SELECT year, month, total_amount
            FROM (
                SELECT year, month, total_amount FROM monthly_data
                UNION ALL
                SELECT year, month, total_amount FROM missing_months
            ) combined
            WHERE TO_DATE(year || '-' || month, 'YYYY-Mon') IN (
                date_trunc('month', $1::timestamp),
                date_trunc('month', $1::timestamp) - interval '1 month'
            )
            ORDER BY
                year DESC,
                TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_total_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(2);
        for row in rows {
            let year: String = row.try_get("year")?;
            let month: String = row.try_get("month")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(MerchantMonthlyTotalAmount {
                year,
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyTotalAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH yearly_data AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time)::integer AS year,
                    COALESCE(SUM(t.amount), 0)::bigint AS total_amount
                FROM
                    transactions t
                INNER JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND (
                        EXTRACT(YEAR FROM t.transaction_time) = $1::integer
                        OR EXTRACT(YEAR FROM t.transaction_time) = $1::integer - 1
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time)
            ), formatted_data AS (
                SELECT
                    year::text,
                    total_amount::bigint
                FROM
                    yearly_data

                UNION ALL

                SELECT
                    $1::text AS year,
                    0::bigint AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $1::integer
                )

                UNION ALL

                SELECT
                    ($1::integer - 1)::text AS year,
                    0::bigint AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $1::integer - 1
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_total_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(2);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(MerchantYearlyTotalAmount { year, total_amount });
        }

        Ok(result)
    }
}
