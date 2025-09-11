use crate::{
    abstract_trait::topup::repository::statsbycard::amount::TopupStatsAmountByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthAmount, TopupYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct TopupStatsAmountByCardRepository {
    db: ConnectionPool,
}

impl TopupStatsAmountByCardRepository {
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
impl TopupStatsAmountByCardRepositoryTrait for TopupStatsAmountByCardRepository {
    async fn get_monthly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(req.year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".to_string()))?
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
                COALESCE(SUM(t.topup_amount), 0)::bigint AS total_amount
            FROM
                months m
            LEFT JOIN
                topups t ON EXTRACT(MONTH FROM t.topup_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.topup_time) = EXTRACT(YEAR FROM m.month)
                AND t.card_number = $1
                AND t.deleted_at IS NULL
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
                error!("❌ Database error in get_monthly_topup_amounts: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
        for row in rows {
            let month: String = row.try_get("month")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TopupMonthAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_amounts(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM t.topup_time)::text AS year,
                SUM(t.topup_amount)::bigint AS total_amount
            FROM
                topups t
            WHERE
                t.deleted_at IS NULL
                AND t.card_number = $1
                AND EXTRACT(YEAR FROM t.topup_time) >= $2 - 4
                AND EXTRACT(YEAR FROM t.topup_time) <= $2
            GROUP BY
                EXTRACT(YEAR FROM t.topup_time)
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_topup_amounts: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TopupYearlyAmount { year, total_amount });
        }

        Ok(result)
    }
}
