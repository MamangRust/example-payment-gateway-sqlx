use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllTransfers {
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
pub struct MonthYearCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthStatusTransferCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct YearStatusTransferCardNumber {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthStatusTransfer {
    #[validate(range(min = 2000, max = 2100, message = "Tahun tidak valid"))]
    pub year: i32,

    #[validate(range(min = 1, max = 12, message = "Bulan harus antara 1 - 12"))]
    pub month: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTransferRequest {
    #[validate(length(min = 1, message = "Transfer from wajib diisi"))]
    pub transfer_from: String,

    #[validate(length(min = 1, message = "Transfer to wajib diisi"))]
    pub transfer_to: String,

    #[validate(range(min = 50000, message = "Minimal transfer 50000"))]
    pub transfer_amount: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTransferRequest {
    pub transfer_id: i32,

    #[validate(length(min = 1, message = "Transfer from wajib diisi"))]
    pub transfer_from: String,

    #[validate(length(min = 1, message = "Transfer to wajib diisi"))]
    pub transfer_to: String,

    #[validate(range(min = 50000, message = "Minimal transfer 50000"))]
    pub transfer_amount: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTransferAmountRequest {
    #[validate(range(min = 1, message = "Transfer ID minimal 1"))]
    pub transfer_id: i32,

    #[validate(range(min = 1, message = "Transfer amount harus lebih dari 0"))]
    pub transfer_amount: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTransferStatus {
    #[validate(range(min = 1, message = "Transfer ID minimal 1"))]
    pub transfer_id: i32,

    #[validate(length(min = 1, message = "Status wajib diisi"))]
    pub status: String,
}
