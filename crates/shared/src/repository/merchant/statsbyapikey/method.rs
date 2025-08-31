use crate::{
    abstract_trait::merchant::repository::statsbyapikey::MerchantStatsMethodByApiKeyRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::MonthYearPaymentMethodApiKey,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyPaymentMethod, MerchantYearlyPaymentMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

pub struct MerchantStatsMethodByApiKeyRepository {
    db: ConnectionPool,
}

impl MerchantStatsMethodByApiKeyRepository {
    pub fn new(db: ConnectionPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MerchantStatsMethodByApiKeyRepositoryTrait for MerchantStatsMethodByApiKeyRepository {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<Vec<MerchantMonthlyPaymentMethod>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let date = NaiveDate::from_ymd_opt(req.year, 1, 1)
            .ok_or_else(|| RepositoryError::Custom("Invalid year".into()))?;

        let year_start: NaiveDateTime = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| RepositoryError::Custom("Invalid datetime".into()))?;

        let results = sqlx::query_as!(
            MerchantMonthlyPaymentMethod,
            r#"
            WITH months AS (
                SELECT generate_series(
                    date_trunc('year', $1::timestamp),
                    date_trunc('year', $1::timestamp) + interval '1 year' - interval '1 day',
                    interval '1 month'
                ) AS month
            ),
            payment_methods AS (
                SELECT DISTINCT payment_method
                FROM transactions
                WHERE deleted_at IS NULL
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS "month!",
                pm.payment_method AS "payment_method!",
                COALESCE(SUM(t.amount), 0)::bigint AS "total_amount!"
            FROM
                months m
            CROSS JOIN
                payment_methods pm
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.payment_method = pm.payment_method
                AND t.deleted_at IS NULL
            LEFT JOIN
                merchants mch ON t.merchant_id = mch.merchant_id
                AND mch.deleted_at IS NULL
                AND mch.api_key = $2
            GROUP BY
                m.month,
                pm.payment_method
            ORDER BY
                m.month,
                pm.payment_method
            "#,
            year_start,
            req.api_key
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethodApiKey,
    ) -> Result<Vec<MerchantYearlyPaymentMethod>, RepositoryError> {
        let mut conn = self.db.acquire().await.map_err(RepositoryError::from)?;

        let results = sqlx::query_as!(
            MerchantYearlyPaymentMethod,
            r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time)::TEXT AS year,
                    t.payment_method,
                    COALESCE(SUM(t.amount), 0)::bigint AS total_amount
                FROM
                    transactions t
                JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND m.api_key = $2
                    AND EXTRACT(YEAR FROM t.transaction_time) >= $1 - 4
                    AND EXTRACT(YEAR FROM t.transaction_time) <= $1
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time),
                    t.payment_method
            )
            SELECT
                year AS "year!",
                payment_method AS "payment_method!",
                total_amount AS "total_amount!"
            FROM
                last_five_years
            ORDER BY
                year,
                payment_method
            "#,
            req.year,
            req.api_key
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(RepositoryError::from)?;

        Ok(results)
    }
}
