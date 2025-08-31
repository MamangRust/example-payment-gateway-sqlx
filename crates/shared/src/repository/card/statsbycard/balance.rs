use crate::{
    abstract_trait::card::repository::CardStatsBalanceByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::card::MonthYearCardNumberCard,
    errors::RepositoryError,
    model::card::{CardMonthBalance, CardYearlyBalance},
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct CardStatsBalanceByCardRepository {
    db: ConnectionPool,
}

impl CardStatsBalanceByCardRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CardStatsBalanceByCardRepositoryTrait for CardStatsBalanceByCardRepository {
    async fn get_monthly_balance(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardMonthBalance>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(req.year, 1, 1)
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
                AND c.card_number = $2
            GROUP BY
                m.month
            ORDER BY
                m.month
            "#,
            year_start,
            req.card_number
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_yearly_balance(
        &self,
        req: &MonthYearCardNumberCard,
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
                    AND c.card_number = $2
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
            req.year,
            req.card_number
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
