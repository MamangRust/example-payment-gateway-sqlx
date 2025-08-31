use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllMerchants {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllMerchantTransactions {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllMerchantTransactionsById {
    #[validate(range(min = 1))]
    pub merchant_id: i32,

    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct FindAllMerchantTransactionsByApiKey {
    #[validate(length(min = 1))]
    pub api_key: String,

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
pub struct MonthYearPaymentMethodApiKey {
    #[validate(length(min = 1, message = "api_key wajib diisi"))]
    pub api_key: String,

    #[validate(range(min = 2000, max = 2100, message = "Tahun harus antara 2000-2100"))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthYearAmountApiKey {
    #[validate(length(min = 1, message = "api_key wajib diisi"))]
    pub api_key: String,

    #[validate(range(min = 2000, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthYearTotalAmountApiKey {
    #[validate(length(min = 1, message = "api_key wajib diisi"))]
    pub api_key: String,

    #[validate(range(min = 2000, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthYearPaymentMethodMerchant {
    #[validate(range(min = 1, message = "merchant_id minimal 1"))]
    pub merchant_id: i32,

    #[validate(range(min = 2000, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthYearAmountMerchant {
    #[validate(range(min = 1))]
    pub merchant_id: i32,

    #[validate(range(min = 2000, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MonthYearTotalAmountMerchant {
    #[validate(range(min = 1))]
    pub merchant_id: i32,

    #[validate(range(min = 2000, max = 2100))]
    pub year: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMerchantRequest {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(range(min = 1))]
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMerchantRequest {
    pub merchant_id: i32,

    #[validate(length(min = 1))]
    pub name: String,

    #[validate(range(min = 1))]
    pub user_id: i32,

    #[validate(length(min = 1))]
    pub status: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMerchantStatus {
    #[validate(range(min = 1))]
    pub merchant_id: i32,

    #[validate(length(min = 1))]
    pub status: String,
}
