use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantModel {
    pub merchant_id: i32,
    pub name: String,
    pub api_key: String,
    pub user_id: i32,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantTransactionsModel {
    pub transaction_id: i32,
    pub card_number: String,
    pub amount: i32,
    pub payment_method: String,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub transaction_time: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantYearlyPaymentMethod {
    pub year: String,
    pub payment_method: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantMonthlyPaymentMethod {
    pub month: String,
    pub payment_method: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantMonthlyAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantYearlyAmount {
    pub year: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantMonthlyTotalAmount {
    pub year: String,
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MerchantYearlyTotalAmount {
    pub year: String,
    pub total_amount: i64,
}
