use crate::{
    abstract_trait::transaction::repository::statsbycard::method::TransactionStatsMethodByCardRepositoryTrait,
    config::ConnectionPool,
    domain::requests::transaction::MonthYearPaymentMethod,
    errors::RepositoryError,
    model::transaction::{TransactionMonthMethod, TransactionYearMethod},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use tracing::error;

pub struct TransactionStatsMethodByCardRepository {
    db: ConnectionPool,
}

impl TransactionStatsMethodByCardRepository {
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
impl TransactionStatsMethodByCardRepositoryTrait for TransactionStatsMethodByCardRepository {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionMonthMethod>, RepositoryError> {
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
            payment_methods AS (
                SELECT DISTINCT payment_method
                FROM transactions
                WHERE deleted_at IS NULL
            )
            SELECT
                TO_CHAR(m.month, 'Mon') AS month,
                pm.payment_method,
                COALESCE(COUNT(t.transaction_id), 0)::int AS total_transactions,
                COALESCE(SUM(t.amount), 0)::bigint AS total_amount
            FROM
                months m
            CROSS JOIN
                payment_methods pm
            LEFT JOIN
                transactions t ON EXTRACT(MONTH FROM t.transaction_time) = EXTRACT(MONTH FROM m.month)
                AND EXTRACT(YEAR FROM t.transaction_time) = EXTRACT(YEAR FROM m.month)
                AND t.payment_method = pm.payment_method
                AND t.card_number = $1
                AND t.deleted_at IS NULL
            GROUP BY
                m.month,
                pm.payment_method
            ORDER BY
                m.month,
                pm.payment_method;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(year_start)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_monthly_method: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let month: String = row.try_get("month")?;
            let payment_method: String = row.try_get("payment_method")?;
            let total_transactions: i32 = row.try_get("total_transactions")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionMonthMethod {
                month,
                payment_method,
                total_transactions,
                total_amount,
            });
        }

        Ok(result)
    }

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<Vec<TransactionYearMethod>, RepositoryError> {
        let mut conn = self.get_conn().await?;

        let sql = r#"
            SELECT
                EXTRACT(YEAR FROM t.transaction_time)::text AS year,
                t.payment_method,
                COUNT(t.transaction_id)::int AS total_transactions,
                SUM(t.amount)::bigint AS total_amount
            FROM
                transactions t
            WHERE
                t.deleted_at IS NULL
                AND t.card_number = $1
                AND EXTRACT(YEAR FROM t.transaction_time) >= $2 - 4
                AND EXTRACT(YEAR FROM t.transaction_time) <= $2
            GROUP BY
                EXTRACT(YEAR FROM t.transaction_time),
                t.payment_method
            ORDER BY
                year;
        "#;

        let rows = sqlx::query(sql)
            .bind(&req.card_number)
            .bind(req.year)
            .fetch_all(&mut *conn)
            .await
            .map_err(|e| {
                error!("❌ Database error in get_yearly_method: {e:?}");
                RepositoryError::Sqlx(e)
            })?;

        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let year: String = row.try_get("year")?;
            let payment_method: String = row.try_get("payment_method")?;
            let total_transactions: i32 = row.try_get("total_transactions")?;
            let total_amount: i64 = row.try_get("total_amount")?;

            result.push(TransactionYearMethod {
                year,
                payment_method,
                total_transactions,
                total_amount,
            });
        }

        Ok(result)
    }
}
