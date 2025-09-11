use crate::{
    abstract_trait::withdraw::repository::statsbycard::amount::WithdrawStatsAmountByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::withdraw::YearMonthCardNumber,
    errors::RepositoryError,
    model::withdraw::{WithdrawMonthlyAmount, WithdrawYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct WithdrawStatsAmountByCardRepository {
    db: ConnectionPool,
}

impl WithdrawStatsAmountByCardRepository {
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
impl WithdrawStatsAmountByCardRepositoryTrait for WithdrawStatsAmountByCardRepository {
    async fn get_monthly_by_card(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<Vec<WithdrawMonthlyAmount>, RepositoryError> {
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
                COALESCE(SUM(w.withdraw_amount), 0)::bigint AS total_amount
            FROM
                months m
            LEFT JOIN
                withdraws w ON EXTRACT(MONTH FROM w.withdraw_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM w.withdraw_time) = EXTRACT(YEAR FROM m.month)
                AND w.card_number = $1
                AND w.deleted_at IS NULL
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
                error!("❌ Database error in get_monthly_by_card_number: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(12);
        for row in rows {
            let month: String = row.try_get("month")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawMonthlyAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_by_card(
        &self,
        req: &YearMonthCardNumber,
    ) -> Result<Vec<WithdrawYearlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM w.withdraw_time)::text AS year,
                SUM(w.withdraw_amount)::bigint AS total_amount
            FROM
                withdraws w
            WHERE
                w.deleted_at IS NULL
                AND w.card_number = $1
                AND EXTRACT(YEAR FROM w.withdraw_time) >= $2 - 4
                AND EXTRACT(YEAR FROM w.withdraw_time) <= $2
            GROUP BY
                EXTRACT(YEAR FROM w.withdraw_time)
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_by_card_number: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(5);
        for row in rows {
            let year: String = row.try_get("year")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(WithdrawYearlyAmount { year, total_amount });
        }

        Ok(result)
    }
}
