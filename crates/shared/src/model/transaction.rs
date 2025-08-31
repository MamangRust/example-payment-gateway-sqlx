use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionModel {
    pub id: i32,
    pub card_number: String,
    pub transaction_no: String,
    pub amount: i64,
    pub payment_method: String,
    pub merchant_id: i32,
    pub transaction_time: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionModelMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionModelYearStatusSuccess {
    pub year: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionModelMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionModelYearStatusFailed {
    pub year: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionMonthMethod {
    pub month: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionYearMethod {
    pub year: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionMonthAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionYearlyAmount {
    pub year: String,
    pub total_amount: i64,
}
