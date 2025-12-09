use crate::{
    abstract_trait::merchant::repository::stats::amount::MerchantStatsAmountRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct MerchantStatsAmountRepository {
    db: ConnectionPool,
}

impl MerchantStatsAmountRepository {
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
impl MerchantStatsAmountRepositoryTrait for MerchantStatsAmountRepository {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError> {
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
            LEFT JOIN
                merchants mch ON t.merchant_id = mch.merchant_id
                AND mch.deleted_at IS NULL
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
                error!("❌ Database error in get_monthly_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
        for row in rows {
            let month: String = row.try_get("month")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(MerchantMonthlyAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time) AS year,
                    SUM(t.amount) AS total_amount
                FROM
                    transactions t
                JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND EXTRACT(YEAR FROM t.transaction_time) >= $1 - 4
                    AND EXTRACT(YEAR FROM t.transaction_time) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time)
            )
            SELECT
                year::text,
                total_amount::bigint
            FROM
                last_five_years
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year_str: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(MerchantYearlyAmount {
                year: year_str,
                total_amount,
            });
        }

        Ok(result)
    }
}
