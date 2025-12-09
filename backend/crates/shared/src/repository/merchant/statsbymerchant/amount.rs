use crate::{
    abstract_trait::merchant::repository::statsbymerchant::amount::MerchantStatsAmountByMerchantRepositoryTrait,
    config::ConnectionPool,
    domain::requests::merchant::MonthYearAmountMerchant,
    errors::RepositoryError,
    model::merchant::{MerchantMonthlyAmount, MerchantYearlyAmount},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct MerchantStatsAmountByMerchantRepository {
    db: ConnectionPool,
}

impl MerchantStatsAmountByMerchantRepository {
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
impl MerchantStatsAmountByMerchantRepositoryTrait for MerchantStatsAmountByMerchantRepository {
    async fn get_monthly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantMonthlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let year_start = NaiveDate::from_ymd_opt(req.year, 1, 1)
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
                COALESCE(SUM(t.amount), 0)::bigint AS total_amount
            FROM
                months m
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.deleted_at IS NULL
                AND t.merchant_id = $2
            LEFT JOIN
                merchants mch ON t.merchant_id = mch.merchant_id
                AND mch.deleted_at IS NULL
            GROUP BY
                m.month
            ORDER BY
                m.month;
        "#;

        let rows = sqlx::query(sql)
            .bind(year_start)
            .bind(req.merchant_id)
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

            result.push(MerchantMonthlyAmount {
                month,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_amount(
        &self,
        req: &MonthYearAmountMerchant,
    ) -> Result<Vec<MerchantYearlyAmount>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            WITH last_five_years AS (
                SELECT
                    EXTRACT(YEAR FROM t.transaction_time) AS year,
                    SUM(t.amount) AS total_amount
                FROM
                    transactions t
                JOIN
                    merchants m ON t.merchant_id = m.merchant_id
                WHERE
                    t.deleted_at IS NULL
                    AND m.deleted_at IS NULL
                    AND t.merchant_id = $1
                    AND EXTRACT(YEAR FROM t.transaction_time) >= $2 - 4
                    AND EXTRACT(YEAR FROM t.transaction_time) <= $2
                GROUP BY
                    EXTRACT(YEAR FROM t.transaction_time)
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
            .bind(req.merchant_id)
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

            result.push(MerchantYearlyAmount { year, total_amount });
        }

        Ok(result)
    }
}
