use crate::{
    abstract_trait::saldo::repository::stats::total::SaldoTotalBalanceRepositoryTrait,
    config::ConnectionPool,
    domain::requests::saldo::MonthTotalSaldoBalance,
    errors::RepositoryError,
    model::saldo::{SaldoMonthTotalBalance, SaldoYearTotalBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Datelike, Days, NaiveDate};
use sqlx::Row;

use tracing::error;

pub struct SaldoTotalBalanceRepository {
    db: ConnectionPool,
}

impl SaldoTotalBalanceRepository {
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
impl SaldoTotalBalanceRepositoryTrait for SaldoTotalBalanceRepository {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<Vec<SaldoMonthTotalBalance>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year = req.year;
        let month = req.month as u32;

        let current_date = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| RepositoryError::Custom("❌ Invalid current date".to_string()))?;

        let prev_date = current_date
            .checked_sub_days(Days::new(1))
            .and_then(|d| NaiveDate::from_ymd_opt(d.year(), d.month(), 1))
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(year - 1, 12, 1).unwrap());

        let last_day_current = current_date
            .checked_add_months(chrono::Months::new(1))
            .and_then(|d| d.checked_sub_days(Days::new(1)))
            .unwrap_or(current_date);

        let last_day_prev = prev_date
            .checked_add_months(chrono::Months::new(1))
            .and_then(|d| d.checked_sub_days(Days::new(1)))
            .unwrap_or(prev_date);

        let sql = r#"
            WITH monthly_data AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at)::integer AS year,
                    EXTRACT(MONTH FROM s.created_at)::integer AS month,
                    COALESCE(SUM(s.total_balance), 0) AS total_balance
                FROM
                    saldos s
                WHERE
                    s.deleted_at IS NULL
                    AND (
                        (s.created_at >= $1::timestamp AND s.created_at <= $2::timestamp)
                        OR (s.created_at >= $3::timestamp AND s.created_at <= $4::timestamp)
                    )
                GROUP BY
                    EXTRACT(YEAR FROM s.created_at),
                    EXTRACT(MONTH FROM s.created_at)
            ), formatted_data AS (
                SELECT
                    year::text,
                    TO_CHAR(TO_DATE(month::text, 'MM'), 'Mon') AS month,
                    total_balance::integer
                FROM
                    monthly_data

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $1::timestamp)::text AS year,
                    TO_CHAR($1::timestamp, 'Mon') AS month,
                    0::integer AS total_balance
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $1::timestamp)::integer
                    AND month = EXTRACT(MONTH FROM $1::timestamp)::integer
                )

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $3::timestamp)::text AS year,
                    TO_CHAR($3::timestamp, 'Mon') AS month,
                    0::integer AS total_balance
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $3::timestamp)::integer
                    AND month = EXTRACT(MONTH FROM $3::timestamp)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY
                year DESC,
                TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(prev_date)
            .bind(last_day_prev)
            .bind(current_date)
            .bind(last_day_current)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_month_total_balance: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let month: String = row.try_get("month")?;
            let total_balance: i64 = row.try_get("total_balance")?;

            result.push(SaldoMonthTotalBalance {
                year,
                month,
                total_balance,
            });
        }

        Ok(result)
    }

    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoYearTotalBalance>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH yearly_data AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at)::integer AS year,
                    COALESCE(SUM(s.total_balance), 0)::integer AS total_balance
                FROM
                    saldos s
                WHERE
                    s.deleted_at IS NULL
                    AND (
                        EXTRACT(YEAR FROM s.created_at) = $1::integer
                        OR EXTRACT(YEAR FROM s.created_at) = $1::integer - 1
                    )
                GROUP BY
                    EXTRACT(YEAR FROM s.created_at)
            ), formatted_data AS (
                SELECT
                    year::text,
                    total_balance::integer
                FROM
                    yearly_data

                UNION ALL

                SELECT
                    $1::text AS year,
                    0::integer AS total_balance
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM yearly_data
                    WHERE year = $1::integer
                )

                UNION ALL

                SELECT
                    ($1::integer - 1)::text AS year,
                    0::integer AS total_balance
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM yearly_data
                    WHERE year = $1::integer - 1
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
                error!("❌ Database error in get_year_total_balance: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year_str: String = row.try_get("year")?;
            let total_balance: i64 = row.try_get("total_balance")?;

            result.push(SaldoYearTotalBalance {
                year: year_str,
                total_balance,
            });
        }

        Ok(result)
    }
}
