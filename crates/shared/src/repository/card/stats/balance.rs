use crate::{
    abstract_trait::card::repository::CardStatsBalanceRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::card::{CardMonthBalance, CardYearlyBalance},
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct CardStatsBalanceRepository {
    db: ConnectionPool,
}

impl CardStatsBalanceRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardStatsBalanceRepositoryTrait for CardStatsBalanceRepository {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardMonthBalance>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            CardMonthBalance,
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
                COALESCE(SUM(s.total_balance), 0)::int AS "total_balance!"
            FROM
                months m
            LEFT JOIN
                saldos s ON EXTRACT(MONTH FROM s.created_at) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM s.created_at) = EXTRACT(YEAR FROM m.month)
                AND s.deleted_at IS NULL
            LEFT JOIN
                cards c ON s.card_number = c.card_number
                AND c.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month
            "#,
            year_start
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardYearlyBalance>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let results = sqlx::query_as!(
            CardYearlyBalance,
            r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at)::TEXT AS year,
                    COALESCE(SUM(s.total_balance), 0)::bigint AS total_balance
                FROM
                    saldos s
                JOIN
                    cards c ON s.card_number = c.card_number
                WHERE
                    s.deleted_at IS NULL 
                    AND c.deleted_at IS NULL
                    AND EXTRACT(YEAR FROM s.created_at) >= $1 - 4
                    AND EXTRACT(YEAR FROM s.created_at) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM s.created_at)
            )
            SELECT
                year AS "year!",
                total_balance AS "total_balance!"
            FROM
                last_five_years
            ORDER BY
                year
            "#,
            year
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
