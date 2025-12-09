use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TopupResponse {
    pub id: i32,
    pub card_number: String,
    pub topup_no: String,
    pub topup_amount: i32,
    pub topup_method: String,
    pub topup_time: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopupResponseDeleteAt {
    pub id: i32,
    pub card_number: String,
    pub topup_no: String,
    pub topup_amount: i32,
    pub topup_method: String,
    pub topup_time: String,
    pub created_at: String,
    pub updated_at: String,
     #[serde(rename = "deleted_At")]
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopupResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i32,
    pub total_success: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupResponseMonthStatusFailed {
    pub year: String,
    pub month: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i32,
    pub total_failed: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupMonthMethodResponse {
    pub month: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupYearlyMethodResponse {
    pub year: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupMonthAmountResponse {
    pub month: String,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TopupYearlyAmountResponse {
    pub year: String,
    pub total_amount: i32,
}
