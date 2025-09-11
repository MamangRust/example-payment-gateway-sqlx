use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct FindAllTransactions {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct FindAllTransactionCardNumber {
    #[validate(length(min = 1))]
    pub card_number: String,
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct MonthYearPaymentMethod {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,
    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct MonthStatusTransaction {
    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
    #[validate(range(min = 1, max = 12))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct YearStatusTransactionCardNumber {
    #[validate(length(min = 1))]
    pub card_number: String,
    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams)]
pub struct MonthStatusTransactionCardNumber {
    #[validate(length(min = 1))]
    pub card_number: String,
    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
    #[validate(range(min = 1, max = 12))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct CreateTransactionRequest {
    #[validate(length(min = 1))]
    pub card_number: String,
    #[validate(range(min = 50000))]
    pub amount: i64,
    #[validate(length(min = 1))]
    pub payment_method: String,
    #[validate(range(min = 1))]
    pub merchant_id: Option<i32>,
    pub transaction_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct UpdateTransactionRequest {
    pub transaction_id: i32,
    #[validate(length(min = 1))]
    pub card_number: String,
    #[validate(range(min = 50000))]
    pub amount: i64,
    #[validate(length(min = 1))]
    pub payment_method: String,
    #[validate(range(min = 1))]
    pub merchant_id: Option<i32>,
    pub transaction_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct UpdateTransactionStatus {
    #[validate(range(min = 1))]
    pub transaction_id: i32,
    #[validate(length(min = 1))]
    pub status: String,
}
