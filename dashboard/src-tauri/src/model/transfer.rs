use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransferResponse {
    pub id: i32,
    pub transfer_no: String,
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: i32,
    pub transfer_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponseDeleteAt {
    pub id: i32,
    pub transfer_no: String,
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: i32,
    pub transfer_time: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponseMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransferMonthAmountResponse {
    pub month: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TransferYearAmountResponse {
    pub year: String,
    pub total_amount: i32,
}
