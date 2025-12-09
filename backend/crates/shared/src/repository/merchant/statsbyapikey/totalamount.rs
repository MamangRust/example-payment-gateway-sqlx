use crate::{
    abstract_trait::merchant::repository::statsbyapikey::totalamount::MerchantStatsTotalAmountByApiKeyRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::MonthYearTotalAmountApiKey,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct MerchantStatsTotalAmountByApiKeyRepository {
    db: ConnectionPool,
}

impl MerchantStatsTotalAmountByApiKeyRepository {
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
impl MerchantStatsTotalAmountByApiKeyRepositoryTrait
    for MerchantStatsTotalAmountByApiKeyRepository
{
    async fn get_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountApiKey,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(req.year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".to_string()))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let sql = r#"
            WITH monthly_data AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time)::integer AS year,
                    EXTRACT(MONTH FROM t.transaction_time)::integer AS month,
                    COALESCE(SUM(t.amount), 0)::bigint AS total_amount
                FROM
                    transactions t
                INNER JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM $1::timestamp)
                    AND m.api_key = $2
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time),
                    EXTRACT(MONTH FROM t.transaction_time)
            ), formatted_data AS (
                SELECT
                    md.year::text,
                    TO_CHAR(TO_DATE(md.month::text, 'MM'), 'Mon') AS month,
                    md.total_amount
                FROM
                    monthly_data md
                UNION ALL
                SELECT
                    EXTRACT(YEAR FROM gs.month)::text AS year,
                    TO_CHAR(gs.month, 'Mon') AS month,
                    0::bigint AS total_amount
                FROM generate_series(
                    date_trunc('year', $1::timestamp),
                    date_trunc('year', $1::timestamp) + interval '11 month',
                    interval '1 month'
                ) AS gs(month)
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data md
                    WHERE md.year = EXTRACT(YEAR FROM gs.month)::integer
                      AND md.month = EXTRACT(MONTH FROM gs.month)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY
                year DESC,
                TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .bind(&req.api_key)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_total_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
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
        req: &MonthYearTotalAmountApiKey,
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
                    AND EXTRACT(YEAR FROM t.transaction_time) >= $1::integer - 4
                    AND EXTRACT(YEAR FROM t.transaction_time) <= $1::integer
                    AND m.api_key = $2
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time)
            ), formatted_data AS (
                SELECT
                    yd.year::text,
                    yd.total_amount
                FROM
                    yearly_data yd
                UNION ALL
                SELECT
                    y::text AS year,
                    0::bigint AS total_amount
                FROM generate_series($1::integer - 4, $1::integer) AS y
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data yd WHERE yd.year = y
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(req.year)
            .bind(&req.api_key)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_total_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(MerchantYearlyTotalAmount { year, total_amount });
        }

        Ok(result)
    }
}
