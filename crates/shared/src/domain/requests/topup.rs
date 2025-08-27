use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllTopups {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllTopupsByCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
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

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthTopupStatus {
    #[validate(range(min = 1900, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthTopupStatusCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1900, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct YearTopupStatusCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1900, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct YearMonthMethod {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 1900, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTopupRequest {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 50000, message = "Minimal topup 50.000"))]
    pub topup_amount: i32,

    #[validate(length(min = 1, message = "Topup method wajib diisi"))]
    pub topup_method: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTopupRequest {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    pub topup_id: Option<i32>,

    #[validate(range(min = 50000, message = "Minimal topup 50.000"))]
    pub topup_amount: i32,

    #[validate(length(min = 1, message = "Topup method wajib diisi"))]
    pub topup_method: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTopupAmount {
    #[validate(range(min = 1, message = "Topup ID wajib diisi"))]
    pub topup_id: i32,

    #[validate(range(min = 50000, message = "Minimal topup 50.000"))]
    pub topup_amount: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTopupStatus {
    #[validate(range(min = 1, message = "Topup ID wajib diisi"))]
    pub topup_id: i32,

    #[validate(length(min = 1, message = "Status wajib diisi"))]
    pub status: String,
}
