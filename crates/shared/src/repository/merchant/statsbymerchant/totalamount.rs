use crate::{
    abstract_trait::merchant::repository::statsbymerchant::MerchantStatsTotalAmountByMerchantRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::MonthYearTotalAmountMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyTotalAmount, MerchantYearlyTotalAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct MerchantStatsTotalAmountByMerchantRepository {
    db: ConnectionPool,
}

impl MerchantStatsTotalAmountByMerchantRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByMerchantRepositoryTrait
    for MerchantStatsTotalAmountByMerchantRepository
{
    async fn get_monthly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
    ) -> Result<Vec<MerchantMonthlyTotalAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(req.year, 1, 1)
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
                AND mch.merchant_id = $2
            GROUP BY
                m.month
            ORDER BY
                m.month DESC
            "#,
            year_start,
            req.merchant_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_yearly_total_amount(
        &self,
        req: &MonthYearTotalAmountMerchant,
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
                AND m.merchant_id = $2
            GROUP BY
                y.year
            ORDER BY
                y.year DESC
            "#,
            req.year,
            req.merchant_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
