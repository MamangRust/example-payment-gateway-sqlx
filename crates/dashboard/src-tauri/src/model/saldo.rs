use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaldoResponse {
    pub id: i32,
    pub card_number: String,
    pub total_balance: i32,
    pub withdraw_amount: i32,
    pub withdraw_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaldoResponseDeleteAt {
    pub id: i32,
    pub card_number: String,
    pub total_balance: i32,
    pub withdraw_amount: i32,
    pub withdraw_time: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaldoMonthTotalBalanceResponse {
    pub month: String,
    pub year: String,
    pub total_balance: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SaldoYearTotalBalanceResponse {
    pub year: String,
    pub total_balance: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SaldoMonthBalanceResponse {
    pub month: String,
    pub total_balance: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SaldoYearBalanceResponse {
    pub year: String,
    pub total_balance: i32,
}
