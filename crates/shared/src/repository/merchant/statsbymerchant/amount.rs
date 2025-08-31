use crate::{
    abstract_trait::merchant::repository::statsbymerchant::MerchantStatsAmountByMerchantRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::MonthYearAmountMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct MerchantStatsAmountByTotalAmountRepository {
    db: ConnectionPool,
}

impl MerchantStatsAmountByTotalAmountRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MerchantStatsAmountByMerchantRepositoryTrait for MerchantStatsAmountByTotalAmountRepository {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(req.year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            MerchantMonthlyAmount,
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
                m.month
            "#,
            year_start,
            req.merchant_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_yearly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let results = sqlx::query_as!(
            MerchantYearlyAmount,
            r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time)::TEXT AS year,
                    COALESCE(SUM(t.amount), 0)::bigint AS total_amount
                FROM
                    transactions t
                JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND m.merchant_id = $2
                    AND EXTRACT(YEAR FROM t.transaction_time) >= $1 - 4
                    AND EXTRACT(YEAR FROM t.transaction_time) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time)
            )
            SELECT
                year AS "year!",
                total_amount AS "total_amount!"
            FROM
                last_five_years
            ORDER BY
                year
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
