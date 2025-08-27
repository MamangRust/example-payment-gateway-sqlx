use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllSaldos {
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

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthTotalSaldoBalance {
    #[validate(range(min = 1900, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSaldoRequest {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1, message = "Total balance wajib diisi"))]
    pub total_balance: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSaldoRequest {
    pub saldo_id: Option<i32>,

    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1, message = "Total balance wajib diisi"))]
    pub total_balance: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSaldoBalance {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 50000, message = "Minimal saldo 50.000"))]
    pub total_balance: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSaldoWithdraw {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 50000, message = "Minimal saldo 50.000"))]
    pub total_balance: i32,

    #[validate(range(min = 0, message = "Withdraw amount tidak boleh negatif"))]
    pub withdraw_amount: Option<i32>,

    pub withdraw_time: Option<NaiveDateTime>,
}
