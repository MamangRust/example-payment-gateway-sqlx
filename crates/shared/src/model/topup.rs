use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupModel {
    pub topup_id: i32,
    pub card_number: String,
    pub topup_no: Uuid,
    pub topup_amount: i64,
    pub topup_method: String,
    pub topup_time: NaiveDateTime,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupModelMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupModelYearStatusSuccess {
    pub year: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupModelMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupModelYearStatusFailed {
    pub year: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupMonthMethod {
    pub month: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupYearlyMethod {
    pub year: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupMonthAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopupYearlyAmount {
    pub year: String,
    pub total_amount: i64,
}
