use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardModel {
    pub id: i32,
    pub user_id: i32,
    pub card_number: String,
    pub card_type: String,
    pub expire_date: String,
    pub cvv: String,
    pub card_provider: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardMonthBalance {
    pub month: String,
    pub total_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardYearlyBalance {
    pub year: String,
    pub total_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardMonthAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CardYearAmount {
    pub year: String,
    pub total_amount: i64,
}
