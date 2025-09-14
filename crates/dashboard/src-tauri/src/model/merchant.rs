use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MerchantResponse {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub api_key: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantResponseDeleteAt {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub api_key: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantTransactionResponse {
    pub id: i32,
    pub card_number: String,
    pub amount: i32,
    pub payment_method: String,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub transaction_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantResponseMonthlyPaymentMethod {
    pub month: String,
    pub payment_method: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantResponseYearlyPaymentMethod {
    pub year: String,
    pub payment_method: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantResponseMonthlyAmount {
    pub month: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MerchantResponseYearlyAmount {
    pub year: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerchantMonthlyTotalAmount {
    pub month: String,
    pub year: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerchantYearlyTotalAmount {
    pub year: String,
    pub total_amount: f64,
}
