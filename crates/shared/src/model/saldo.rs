use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaldoModel {
    pub saldo_id: i32,
    pub card_number: String,
    pub total_balance: i32,
    pub withdraw_amount: Option<i32>,
    pub withdraw_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaldoMonthTotalBalance {
    pub year: String,
    pub month: String,
    pub total_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaldoYearTotalBalance {
    pub year: String,
    pub total_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaldoMonthSaldoBalance {
    pub month: String,
    pub total_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SaldoYearSaldoBalance {
    pub year: String,
    pub total_balance: i64,
}
