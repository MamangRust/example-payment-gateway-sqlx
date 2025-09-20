use crate::utils::deserialize_date_only;
use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllCards {
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
pub struct MonthYearCardNumberCard {
    #[validate(length(min = 1, message = "Card number wajib diisi"))]
    pub card_number: String,

    #[validate(range(min = 2000, max = 2100, message = "Tahun harus antara 2000 dan 2100"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCardRequest {
    #[validate(range(min = 1, message = "User ID minimal 1"))]
    pub user_id: i32,

    #[validate(length(min = 1, message = "Card type wajib diisi"))]
    pub card_type: String,

    #[serde(deserialize_with = "deserialize_date_only")]
    pub expire_date: NaiveDate,

    #[validate(length(min = 1, message = "CVV wajib diisi"))]
    pub cvv: String,

    #[validate(length(min = 1, message = "Card provider wajib diisi"))]
    pub card_provider: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCardRequest {
    pub card_id: Option<i32>,

    #[validate(range(min = 1, message = "User ID minimal 1"))]
    pub user_id: i32,

    #[validate(length(min = 1, message = "Card type wajib diisi"))]
    pub card_type: String,

    #[serde(deserialize_with = "deserialize_date_only")]
    pub expire_date: NaiveDate,

    #[validate(length(min = 1, message = "CVV wajib diisi"))]
    pub cvv: String,

    #[validate(length(min = 1, message = "Card provider wajib diisi"))]
    pub card_provider: String,
}
