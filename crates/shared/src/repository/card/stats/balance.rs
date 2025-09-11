use crate::{
    abstract_trait::card::repository::stats::balance::CardStatsBalanceRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::card::{CardMonthBalance, CardYearlyBalance},
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct CardStatsBalanceRepository {
    db: ConnectionPool,
}

impl CardStatsBalanceRepository {
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
impl CardStatsBalanceRepositoryTrait for CardStatsBalanceRepository {
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardMonthBalance>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("❌ Invalid year".to_string()))?
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
                COALESCE(SUM(s.total_balance), 0)::bigint AS total_balance
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
                m.month;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_balance: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
        for row in rows {
            let month: String = row.try_get("month")?;
            let total_balance: i64 = row.try_get("total_balance")?;

            result.push(CardMonthBalance {
                month,
                total_balance,
            });
        }

        Ok(result)
    }

    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<Vec<CardYearlyBalance>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM s.created_at) AS year,
                    SUM(s.total_balance) AS total_balance
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
                year::text,
                total_balance::bigint
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
                error!("❌ Database error in get_yearly_balance: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year_str: String = row.try_get("year")?;
            let total_balance: i64 = row.try_get("total_balance")?;

            result.push(CardYearlyBalance {
                year: year_str,
                total_balance,
            });
        }

        Ok(result)
    }
}
