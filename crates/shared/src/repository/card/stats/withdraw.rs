use crate::{
    abstract_trait::card::repository::CardStatsWithdrawRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct CardStatsWithdrawRepository {
    db: ConnectionPool,
}

impl CardStatsWithdrawRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardStatsWithdrawRepositoryTrait for CardStatsWithdrawRepository {
    async fn get_monthly_amount(&self, year: i32) -> Result<Vec<CardMonthAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;
        
        let date = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            CardMonthAmount,
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
                COALESCE(SUM(w.withdraw_amount), 0)::int AS "total_amount!"
            FROM
                months m
            LEFT JOIN
                withdraws w ON EXTRACT(MONTH FROM w.withdraw_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM w.withdraw_time) = EXTRACT(YEAR FROM m.month)
                AND w.deleted_at IS NULL
            LEFT JOIN
                cards c ON w.card_number = c.card_number
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

    async fn get_yearly_amount(&self, year: i32) -> Result<Vec<CardYearAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;
        
        let results = sqlx::query_as!(
            CardYearAmount,
            r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM w.withdraw_time)::TEXT AS year,
                    COALESCE(SUM(w.withdraw_amount), 0)::bigint AS total_amount
                FROM
                    withdraws w
                JOIN
                    cards c ON w.card_number = c.card_number
                WHERE
                    w.deleted_at IS NULL
                    AND c.deleted_at IS NULL
                    AND EXTRACT(YEAR FROM w.withdraw_time) >= $1 - 4
                    AND EXTRACT(YEAR FROM w.withdraw_time) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM w.withdraw_time)
            )
            SELECT
                year AS "year!",
                total_amount AS "total_amount!"
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
