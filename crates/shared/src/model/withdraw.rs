use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawModel {
    pub withdraw_id: i32,
    pub withdraw_no: String,
    pub card_number: String,
    pub withdraw_amount: i64,
    pub withdraw_time: NaiveDateTime,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawModelMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawModelYearStatusSuccess {
    pub year: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawModelMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawModelYearStatusFailed {
    pub year: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawMonthlyAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawYearlyAmount {
    pub year: String,
    pub total_amount: i64,
}
