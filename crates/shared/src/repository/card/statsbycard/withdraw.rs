use crate::{
    abstract_trait::card::repository::statsbycard::withdraw::CardStatsWithdrawByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::card::MonthYearCardNumberCard,
    errors::RepositoryError,
    model::card::{CardMonthAmount, CardYearAmount},
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct CardStatsWithdrawByCardRepository {
    db: ConnectionPool,
}

impl CardStatsWithdrawByCardRepository {
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
impl CardStatsWithdrawByCardRepositoryTrait for CardStatsWithdrawByCardRepository {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardMonthAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(req.year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("❌ Invalid year".to_string()))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let sql = r#"
            WITH months AS (
                SELECT generate_series(
                    date_trunc('year', $2::timestamp),
                    date_trunc('year', $2::timestamp) + interval '1 year' - interval '1 day',
                    interval '1 month'
                ) AS month
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS month,
                COALESCE(SUM(w.withdraw_amount), 0)::bigint AS total_amount
            FROM
                months m
            LEFT JOIN
                withdraws w ON EXTRACT(MONTH FROM w.withdraw_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM w.withdraw_time) = EXTRACT(YEAR FROM m.month)
                AND w.deleted_at IS NULL
                AND w.card_number = $1
            LEFT JOIN
                cards c ON w.card_number = c.card_number
                AND c.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
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

            result.push(CardMonthAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_amount(
        &self,
        req: &MonthYearCardNumberCard,
    ) -> Result<Vec<CardYearAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM w.withdraw_time) AS year,
                    SUM(w.withdraw_amount) AS total_amount
                FROM
                    withdraws w
                JOIN
                    cards c ON w.card_number = c.card_number
                WHERE
                    w.deleted_at IS NULL
                    AND c.deleted_at IS NULL
                    AND w.card_number = $1
                    AND EXTRACT(YEAR FROM w.withdraw_time) >= $2 - 4
                    AND EXTRACT(YEAR FROM w.withdraw_time) <= $2
                GROUP BY
                    EXTRACT(YEAR FROM w.withdraw_time)
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
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_amount: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(CardYearAmount { year, total_amount });
        }

        Ok(result)
    }
}
