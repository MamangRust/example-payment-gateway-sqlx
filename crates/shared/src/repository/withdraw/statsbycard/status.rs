use crate::{
    abstract_trait::withdraw::repository::statsbycard::status::WithdrawStatsStatusByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::withdraw::{MonthStatusWithdrawCardNumber, YearStatusWithdrawCardNumber},
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

pub struct WithdrawStatsStatusByCardRepository {
    db: ConnectionPool,
}

impl WithdrawStatsStatusByCardRepository {
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
impl WithdrawStatsStatusByCardRepositoryTrait for WithdrawStatsStatusByCardRepository {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
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
                    AND t.card_number = $1
                    AND (
                        (t.withdraw_time >= $2::timestamp AND t.withdraw_time <= $3::timestamp)
                        OR (t.withdraw_time >= $4::timestamp AND t.withdraw_time <= $5::timestamp)
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
                    EXTRACT(YEAR FROM $2::timestamp)::text AS year,
                    TO_CHAR($2::timestamp, 'Mon') AS month,
                    0 AS total_success,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $2::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $2::timestamp)::integer
                )

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $4::timestamp)::text AS year,
                    TO_CHAR($4::timestamp, 'Mon') AS month,
                    0 AS total_success,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $4::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $4::timestamp)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC, TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(prev_date)
            .bind(last_day_prev)
            .bind(current_date)
            .bind(last_day_current)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("Database error in get_month_status_success_by_card_number: {e:?}",);
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

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
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
                    AND t.card_number = $1
                    AND (
                        EXTRACT(YEAR FROM t.withdraw_time) = $2::integer
                        OR EXTRACT(YEAR FROM t.withdraw_time) = $2::integer - 1
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
                    $2::text AS year,
                    0::integer AS total_success,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $2::integer
                )

                UNION ALL

                SELECT
                    ($2::integer - 1)::text AS year,
                    0::integer AS total_success,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $2::integer - 1
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("Database error in get_yearly_status_success_by_card_number: {e:?}",);
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

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusWithdrawCardNumber,
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
                    AND t.card_number = $1
                    AND (
                        (t.withdraw_time >= $2::timestamp AND t.withdraw_time <= $3::timestamp)
                        OR (t.withdraw_time >= $4::timestamp AND t.withdraw_time <= $5::timestamp)
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
                    EXTRACT(YEAR FROM $2::timestamp)::text AS year,
                    TO_CHAR($2::timestamp, 'Mon') AS month,
                    0 AS total_failed,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $2::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $2::timestamp)::integer
                )

                UNION ALL

                SELECT
                    EXTRACT(YEAR FROM $4::timestamp)::text AS year,
                    TO_CHAR($4::timestamp, 'Mon') AS month,
                    0 AS total_failed,
                    0 AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM monthly_data
                    WHERE year = EXTRACT(YEAR FROM $4::timestamp)::integer
                      AND month = EXTRACT(MONTH FROM $4::timestamp)::integer
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC, TO_DATE(month, 'Mon') DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(prev_date)
            .bind(last_day_prev)
            .bind(current_date)
            .bind(last_day_current)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_month_status_failed_by_card_number: {e:?}");
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

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusWithdrawCardNumber,
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
                    AND t.card_number = $1
                    AND (
                        EXTRACT(YEAR FROM t.withdraw_time) = $2::integer
                        OR EXTRACT(YEAR FROM t.withdraw_time) = $2::integer - 1
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
                    $2::text AS year,
                    0::integer AS total_failed,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $2::integer
                )

                UNION ALL

                SELECT
                    ($2::integer - 1)::text AS year,
                    0::integer AS total_failed,
                    0::integer AS total_amount
                WHERE NOT EXISTS (
                    SELECT 1 FROM yearly_data WHERE year = $2::integer - 1
                )
            )
            SELECT * FROM formatted_data
            ORDER BY year DESC;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_status_failed_by_card_number: {e:?}");
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
