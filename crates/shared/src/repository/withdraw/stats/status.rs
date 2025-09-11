use crate::{
    abstract_trait::withdraw::repository::stats::status::WithdrawStatsStatusRepositoryTrait,
    config::ConnectionPool,
    domain::requests::withdraw::MonthStatusWithdraw,
    errors::RepositoryError,
    model::withdraw::{
        WithdrawModelMonthStatusFailed, WithdrawModelMonthStatusSuccess,
        WithdrawModelYearStatusFailed, WithdrawModelYearStatusSuccess,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Datelike, Days, NaiveDate};
use sqlx::Row;
use tracing::error;

pub struct WithdrawStatsStatusRepository {
    db: ConnectionPool,
}

impl WithdrawStatsStatusRepository {
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
impl WithdrawStatsStatusRepositoryTrait for WithdrawStatsStatusRepository {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<Vec<WithdrawModelMonthStatusSuccess>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year = req.year;
        let month = req.month as u32;

        let current_date = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid current date".to_string()))?;

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
                    EXTRACT(YEAR FROM t.withdraw_time)::integer AS year,
                    EXTRACT(MONTH FROM t.withdraw_time)::integer AS month,
                    COUNT(*) AS total_success,
                    COALESCE(SUM(t.withdraw_amount), 0)::bigint AS total_amount
                FROM
                    withdraws t
                WHERE
                    t.deleted_at IS NULL
                    AND t.status = 'success'
                    AND (
                        (t.withdraw_time >= $1::timestamp AND t.withdraw_time <= $2::timestamp)
                        OR (t.withdraw_time >= $3::timestamp AND t.withdraw_time <= $4::timestamp)
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.withdraw_time),
                    EXTRACT(MONTH FROM t.withdraw_time)
            ), formatted_data AS (
                SELECT
                    year::text,
                    TO_CHAR(TO_DATE(month::text, 'MM'), 'Mon') AS month,
                    total_success::integer,
                    total_amount::bigint
                FROM
                    monthly_data

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $1::timestamp)::text AS year,
                    TO_CHAR($1::timestamp, 'Mon') AS month,
                    0 AS total_success,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $1::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $1::timestamp)::integer
                )

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $3::timestamp)::text AS year,
                    TO_CHAR($3::timestamp, 'Mon') AS month,
                    0 AS total_success,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $3::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $3::timestamp)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC, TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(prev_date)
            .bind(last_day_prev)
            .bind(current_date)
            .bind(last_day_current)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_month_status_success: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let month: String = row.try_get("month")?;
            let total_success: i32 = row.try_get("total_success")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawModelMonthStatusSuccess {
                year,
                month,
                total_success,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawModelYearStatusSuccess>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH yearly_data AS (
                SELECT
                    EXTRACT(YEAR FROM t.withdraw_time)::integer AS year,
                    COUNT(*) AS total_success,
                    COALESCE(SUM(t.withdraw_amount), 0)::bigint AS total_amount
                FROM
                    withdraws t
                WHERE
                    t.deleted_at IS NULL
                    AND t.status = 'success'
                    AND (
                        EXTRACT(YEAR FROM t.withdraw_time) = $1::integer
                        OR EXTRACT(YEAR FROM t.withdraw_time) = $1::integer - 1
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.withdraw_time)
            ), formatted_data AS (
                SELECT
                    year::text,
                    total_success::integer,
                    total_amount::bigint
                FROM
                    yearly_data

                UNION ALL

                SELECT
                    $1::text AS year,
                    0::integer AS total_success,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $1::integer
                )

                UNION ALL

                SELECT
                    ($1::integer - 1)::text AS year,
                    0::integer AS total_success,
                    0::integer AS total_amount
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
                error!("❌ Database error in get_yearly_status_success: {}", e);
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_success: i32 = row.try_get("total_success")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawModelYearStatusSuccess {
                year,
                total_success,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusWithdraw,
    ) -> Result<Vec<WithdrawModelMonthStatusFailed>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year = req.year;
        let month = req.month as u32;

        let current_date = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid current date".to_string()))?;

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
                    EXTRACT(YEAR FROM t.withdraw_time)::integer AS year,
                    EXTRACT(MONTH FROM t.withdraw_time)::integer AS month,
                    COUNT(*) AS total_failed,
                    COALESCE(SUM(t.withdraw_amount), 0)::bigint AS total_amount
                FROM
                    withdraws t
                WHERE
                    t.deleted_at IS NULL
                    AND t.status = 'failed'
                    AND (
                        (t.withdraw_time >= $1::timestamp AND t.withdraw_time <= $2::timestamp)
                        OR (t.withdraw_time >= $3::timestamp AND t.withdraw_time <= $4::timestamp)
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.withdraw_time),
                    EXTRACT(MONTH FROM t.withdraw_time)
            ), formatted_data AS (
                SELECT
                    year::text,
                    TO_CHAR(TO_DATE(month::text, 'MM'), 'Mon') AS month,
                    total_failed::integer,
                    total_amount::bigint
                FROM
                    monthly_data

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $1::timestamp)::text AS year,
                    TO_CHAR($1::timestamp, 'Mon') AS month,
                    0 AS total_failed,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $1::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $1::timestamp)::integer
                )

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $3::timestamp)::text AS year,
                    TO_CHAR($3::timestamp, 'Mon') AS month,
                    0 AS total_failed,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $3::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $3::timestamp)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC, TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(prev_date)
            .bind(last_day_prev)
            .bind(current_date)
            .bind(last_day_current)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_month_status_failed: {}", e);
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let month: String = row.try_get("month")?;
            let total_failed: i32 = row.try_get("total_failed")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawModelMonthStatusFailed {
                year,
                month,
                total_failed,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<Vec<WithdrawModelYearStatusFailed>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH yearly_data AS (
                SELECT
                    EXTRACT(YEAR FROM t.withdraw_time)::integer AS year,
                    COUNT(*) AS total_failed,
                    COALESCE(SUM(t.withdraw_amount), 0)::bigint AS total_amount
                FROM
                    withdraws t
                WHERE
                    t.deleted_at IS NULL
                    AND t.status = 'failed'
                    AND (
                        EXTRACT(YEAR FROM t.withdraw_time) = $1::integer
                        OR EXTRACT(YEAR FROM t.withdraw_time) = $1::integer - 1
                    )
                GROUP BY
                    EXTRACT(YEAR FROM t.withdraw_time)
            ), formatted_data AS (
                SELECT
                    year::text,
                    total_failed::integer,
                    total_amount::bigint
                FROM
                    yearly_data

                UNION ALL

                SELECT
                    $1::text AS year,
                    0::integer AS total_failed,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $1::integer
                )

                UNION ALL

                SELECT
                    ($1::integer - 1)::text AS year,
                    0::integer AS total_failed,
                    0::integer AS total_amount
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
                error!("❌ Database error in get_yearly_status_failed: {}", e);
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_failed: i32 = row.try_get("total_failed")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawModelYearStatusFailed {
                year,
                total_failed,
                total_amount,
            });
        }

        Ok(result)
    }
}
