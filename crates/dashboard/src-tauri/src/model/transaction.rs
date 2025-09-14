use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: i32,
    pub transaction_no: String,
    pub card_number: String,
    pub amount: i32,
    pub payment_method: String,
    pub merchant_id: i32,
    pub transaction_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseDeleteAt {
    pub id: i32,
    pub transaction_no: String,
    pub card_number: String,
    pub amount: i32,
    pub payment_method: String,
    pub merchant_id: i32,
    pub transaction_time: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionMonthMethodResponse {
    pub month: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionYearMethodResponse {
    pub year: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionMonthAmountResponse {
    pub month: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionYearlyAmountResponse {
    pub year: String,
    pub total_amount: i32,
}
