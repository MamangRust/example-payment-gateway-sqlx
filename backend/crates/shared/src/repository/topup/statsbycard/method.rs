use crate::{
    abstract_trait::topup::repository::statsbycard::method::TopupStatsMethodByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::topup::YearMonthMethod,
    errors::RepositoryError,
    model::topup::{TopupMonthMethod, TopupYearlyMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct TopupStatsMethodByCardRepository {
    db: ConnectionPool,
}

impl TopupStatsMethodByCardRepository {
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
impl TopupStatsMethodByCardRepositoryTrait for TopupStatsMethodByCardRepository {
    async fn get_monthly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupMonthMethod>, RepositoryError> {
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
            ),
            topup_methods AS (
                SELECT DISTINCT topup_method
                FROM topups
                WHERE deleted_at IS NULL
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS month,
                tm.topup_method,
                COALESCE(COUNT(t.topup_id), 0)::int AS total_topups,
                COALESCE(SUM(t.topup_amount), 0)::bigint AS total_amount
            FROM
                months m
            CROSS JOIN
                topup_methods tm
            LEFT JOIN
                topups t ON EXTRACT(MONTH FROM t.topup_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.topup_time) = EXTRACT(YEAR FROM m.month)
                AND t.topup_method = tm.topup_method
                AND t.card_number = $1
                AND t.deleted_at IS NULL
            GROUP BY
                m.month,
                tm.topup_method
            ORDER BY
                m.month,
                tm.topup_method;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_topup_methods: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let month: String = row.try_get("month")?;
            let topup_method: String = row.try_get("topup_method")?;
            let total_topups: i32 = row.try_get("total_topups")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TopupMonthMethod {
                month,
                topup_method,
                total_topups,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<Vec<TopupYearlyMethod>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM t.topup_time)::text AS year,
                t.topup_method,
                COUNT(t.topup_id)::int AS total_topups,
                SUM(t.topup_amount)::bigint AS total_amount
            FROM
                topups t
            WHERE
                t.deleted_at IS NULL
                AND t.card_number = $1
                AND EXTRACT(YEAR FROM t.topup_time) >= $2 - 4
                AND EXTRACT(YEAR FROM t.topup_time) <= $2
            GROUP BY
                EXTRACT(YEAR FROM t.topup_time),
                t.topup_method
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_topup_methods: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let topup_method: String = row.try_get("topup_method")?;
            let total_topups: i32 = row.try_get("total_topups")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TopupYearlyMethod {
                year,
                topup_method,
                total_topups,
                total_amount,
            });
        }

        Ok(result)
    }
}
