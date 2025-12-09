use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawResponse {
    pub withdraw_id: i32,
    pub withdraw_no: String,
    pub card_number: String,
    pub withdraw_amount: i32,
    pub withdraw_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponseDeleteAt {
    pub withdraw_id: i32,
    pub withdraw_no: String,
    pub card_number: String,
    pub withdraw_amount: i32,
    pub withdraw_time: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponseMonthStatusFailed {
    pub year: String,
    pub total_amount: i32,
    pub month: String,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawMonthlyAmountResponse {
    pub month: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawYearlyAmountResponse {
    pub year: String,
    pub total_amount: i32,
}
