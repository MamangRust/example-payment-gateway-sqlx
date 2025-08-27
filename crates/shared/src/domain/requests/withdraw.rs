use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllWithdraws {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllWithdrawCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(length(min = 1, message = "Search wajib diisi"))]
    pub search: String,

    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct YearMonthCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1, message = "Tahun wajib diisi"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthStatusWithdraw {
    #[validate(range(min = 1, message = "Tahun wajib diisi"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthStatusWithdrawCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1, message = "Tahun wajib diisi"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct YearStatusWithdrawCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1, message = "Tahun wajib diisi"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateWithdrawRequest {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 50000, message = "Minimal withdraw 50000"))]
    pub withdraw_amount: i32,

    pub withdraw_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateWithdrawRequest {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    pub withdraw_id: Option<i32>,

    #[validate(range(min = 50000, message = "Minimal withdraw 50000"))]
    pub withdraw_amount: i32,

    pub withdraw_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateWithdrawStatus {
    #[validate(range(min = 1, message = "Withdraw ID wajib diisi"))]
    pub withdraw_id: i32,

    #[validate(length(min = 1, message = "Status wajib diisi"))]
    pub status: String,
}
