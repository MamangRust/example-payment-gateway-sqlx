use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferModel {
    pub transfer_id: i32,
    pub transfer_no: Uuid,
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: i32,
    pub transfer_time: NaiveDateTime,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferModelMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferModelYearStatusSuccess {
    pub year: String,
    pub total_success: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferModelMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferModelYearStatusFailed {
    pub year: String,
    pub total_failed: i32,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferMonthAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TransferYearAmount {
    pub year: String,
    pub total_amount: i64,
}
