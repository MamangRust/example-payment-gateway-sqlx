use crate::{
    abstract_trait::merchant::repository::stats::MerchantStatsTotalAmountRepositoryTrait,
    config::ConnectionPool,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct MerchantStatsTotalAmountRepository {
    db: ConnectionPool,
}

impl MerchantStatsTotalAmountRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountRepositoryTrait for MerchantStatsTotalAmountRepository {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            MerchantMonthlyTotalAmount,
            r#"
            WITH months AS (
                SELECT generate_series(
                    date_trunc('year', $1::timestamp),
                    date_trunc('year', $1::timestamp) + interval '1 year' - interval '1 day',
                    interval '1 month'
                ) AS month
            )
            SELECT
                EXTRACT(YEAR FROM m.month)::text AS "year!",
                TO_CHAR(m.month, 'Mon') AS "month!",
                COALESCE(SUM(t.amount), 0)::bigint AS "total_amount!"
            FROM
                months m
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.deleted_at IS NULL
            LEFT JOIN
                merchants mch ON t.merchant_id = mch.merchant_id
                AND mch.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month DESC
            "#,
            year_start
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<Vec<MerchantYearlyTotalAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let results = sqlx::query_as!(
            MerchantYearlyTotalAmount,
            r#"
            WITH years AS (
                SELECT generate_series($1 - 4, $1) AS year
            )
            SELECT
                y.year::text AS "year!",
                COALESCE(SUM(t.amount), 0)::bigint AS "total_amount!"
            FROM
                years y
            LEFT JOIN
                transactions t ON EXTRACT(YEAR FROM t.transaction_time) = y.year
                AND t.deleted_at IS NULL
            LEFT JOIN
                merchants m ON t.merchant_id = m.merchant_id
                AND m.deleted_at IS NULL
            GROUP BY
                y.year
            ORDER BY
                y.year DESC
            "#,
            year
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
