use crate::{
    abstract_trait::saldo::repository::stats::balance::SaldoBalanceRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::saldo::{SaldoMonthSaldoBalance, SaldoYearSaldoBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct SaldoBalanceRepository {
    db: ConnectionPool,
}

impl SaldoBalanceRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SaldoBalanceRepositoryTrait for SaldoBalanceRepository {
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoMonthSaldoBalance>, RepositoryError> {
        let date = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            SaldoMonthSaldoBalance,
            r#"
            WITH months AS (
                SELECT generate_series(
                    date_trunc('year', $1::timestamp),
                    date_trunc('year', $1::timestamp) + interval '1 year' - interval '1 day',
                    interval '1 month'
                ) AS month
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS "month!",
                COALESCE(SUM(s.total_balance), 0)::bigint AS "total_balance!"
            FROM
                months m
            LEFT JOIN
                saldos s ON EXTRACT(MONTH FROM s.created_at) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM s.created_at) = EXTRACT(YEAR FROM m.month)
                AND s.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month
            "#,
            year_start
        )
        .fetch_all(&self.db)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<Vec<SaldoYearSaldoBalance>, RepositoryError> {
        let results = sqlx::query_as!(
            SaldoYearSaldoBalance,
            r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at)::int AS year,
                    COALESCE(SUM(s.total_balance), 0)::bigint AS total_balance
                FROM
                    saldos s
                WHERE
                    s.deleted_at IS NULL
                    AND EXTRACT(YEAR FROM s.created_at) >= $1 - 4
                    AND EXTRACT(YEAR FROM s.created_at) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM s.created_at)
            )
            SELECT
                year::text AS "year!",
                total_balance AS "total_balance!"
            FROM
                last_five_years
            ORDER BY
                year
            "#,
            year
        )
        .fetch_all(&self.db)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
