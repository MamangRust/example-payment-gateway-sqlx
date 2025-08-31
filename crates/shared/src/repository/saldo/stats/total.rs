use crate::{
    abstract_trait::saldo::repository::stats::total::SaldoTotalBalanceRepositoryTrait,
    config::ConnectionPool,
    domain::requests::saldo::MonthTotalSaldoBalance,
    errors::RepositoryError,
    model::saldo::{SaldoMonthTotalBalance, SaldoYearTotalBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Days, NaiveDate};

use tracing::error;

pub struct SaldoTotalBalanceRepository {
    db: ConnectionPool,
}

impl SaldoTotalBalanceRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SaldoTotalBalanceRepositoryTrait for SaldoTotalBalanceRepository {
    async fn get_month_total_balance(
        &self,
        req: &MonthTotalSaldoBalance,
    ) -> Result<Vec<SaldoMonthTotalBalance>, RepositoryError> {
        let year = req.year;
        let month = req.month as u32;

        if month < 1 || month > 12 {
            return Err(RepositoryError::Custom(
                "Bulan harus antara 1 dan 12".to_string(),
            ));
        }

        let current_month_start = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or(RepositoryError::Custom("Tanggal tidak valid".to_string()))?;

        let next_month_start = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
        }
        .ok_or(RepositoryError::Custom("Tanggal tidak valid".to_string()))?;

        let last_day_current_month = next_month_start
            .checked_sub_days(Days::new(1))
            .ok_or(RepositoryError::Custom("Tanggal tidak valid".to_string()))?;

        let prev_month_start = if month == 1 {
            NaiveDate::from_ymd_opt(year - 1, 12, 1)
        } else {
            NaiveDate::from_ymd_opt(year, month - 1, 1)
        }
        .ok_or(RepositoryError::Custom("Tanggal tidak valid".to_string()))?;

        let last_day_prev_month = current_month_start
            .checked_sub_days(Days::new(1))
            .ok_or(RepositoryError::Custom("Tanggal tidak valid".to_string()))?;

        let start_current = current_month_start.and_hms_opt(0, 0, 0).unwrap();
        let end_current = last_day_current_month.and_hms_opt(23, 59, 59).unwrap();
        let start_prev = prev_month_start.and_hms_opt(0, 0, 0).unwrap();
        let end_prev = last_day_prev_month.and_hms_opt(23, 59, 59).unwrap();

        let rows = sqlx::query!(
            r#"
            WITH monthly_data AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at)::integer AS year,
                    EXTRACT(MONTH FROM s.created_at)::integer AS month,
                    COALESCE(SUM(s.total_balance), 0)::integer AS total_balance
                FROM saldos s
                WHERE s.deleted_at IS NULL
                AND (
                    (s.created_at >= $1 AND s.created_at <= $2)
                    OR (s.created_at >= $3 AND s.created_at <= $4)
                )
                GROUP BY EXTRACT(YEAR FROM s.created_at), EXTRACT(MONTH FROM s.created_at)
            ), formatted_data AS (
                SELECT year::text, TO_CHAR(TO_DATE(month::text, 'MM'), 'Mon') AS month, total_balance::integer
                FROM monthly_data
                UNION ALL
                SELECT EXTRACT(YEAR FROM $1)::text, TO_CHAR($1, 'Mon'), 0::integer
                WHERE NOT EXISTS (SELECT 1 FROM monthly_data WHERE year = EXTRACT(YEAR FROM $1)::integer AND month = EXTRACT(MONTH FROM $1)::integer)
                UNION ALL
                SELECT EXTRACT(YEAR FROM $3)::text, TO_CHAR($3, 'Mon'), 0::integer
                WHERE NOT EXISTS (SELECT 1 FROM monthly_data WHERE year = EXTRACT(YEAR FROM $3)::integer AND month = EXTRACT(MONTH FROM $3)::integer)
            )
            SELECT 
                year as "year!",
                month as "month!",
                total_balance as "total_balance!" 
            FROM formatted_data
            ORDER BY year DESC, TO_DATE(month, 'Mon') DESC
            "#,
            start_current,
            end_current,
            start_prev,
            end_prev
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch monthly saldo balance: {:?}", e);
            RepositoryError::Sqlx(e.into())
        })?;

        let result: Vec<SaldoMonthTotalBalance> = rows
            .into_iter()
            .map(|r| SaldoMonthTotalBalance {
                year: r.year,
                month: r.month,
                total_balance: r.total_balance as i64,
            })
            .collect();

        Ok(result)
    }

    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoYearTotalBalance>, RepositoryError> {
        let prev_year = year - 1;

        let rows = sqlx::query!(
            r#"
        WITH yearly_data AS (
            SELECT
                (EXTRACT(YEAR FROM s.created_at))::INT AS year,
                CAST(COALESCE(SUM(s.total_balance), 0) AS BIGINT) AS total_balance
            FROM
                saldos s
            WHERE
                s.deleted_at IS NULL
                AND (
                    (EXTRACT(YEAR FROM s.created_at))::INT = $1
                    OR (EXTRACT(YEAR FROM s.created_at))::INT = $2
                )
            GROUP BY
                (EXTRACT(YEAR FROM s.created_at))::INT
        ), formatted_data AS (
            SELECT
                year::TEXT,
                total_balance
            FROM
                yearly_data

            UNION ALL

            SELECT
                $1::TEXT AS year,
                0::BIGINT AS total_balance
            WHERE NOT EXISTS (
                SELECT 1 FROM yearly_data WHERE year = $1
            )

            UNION ALL

            SELECT
                $2::TEXT AS year,
                0::BIGINT AS total_balance
            WHERE NOT EXISTS (
                SELECT 1 FROM yearly_data WHERE year = $2
            )
        )
        SELECT 
            COALESCE(year::TEXT, '') AS year, 
            COALESCE(total_balance, 0) AS total_balance
        FROM formatted_data
        ORDER BY year DESC
        "#,
            year,
            prev_year
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            error!("❌ Failed to fetch yearly saldo balance: {:?}", e);
            RepositoryError::Sqlx(e.into())
        })?
        .into_iter()
        .map(|row| SaldoYearTotalBalance {
            year: row.year.unwrap_or_default(),
            total_balance: row.total_balance.unwrap_or(0),
        })
        .collect();

        Ok(rows)
    }
}
