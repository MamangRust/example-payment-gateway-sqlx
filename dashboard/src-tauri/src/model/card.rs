use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CardResponse {
    pub id: i32,
    pub user_id: i32,
    pub card_number: String,
    pub card_type: String,
    pub expire_date: String,
    pub cvv: String,
    pub card_provider: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CardResponseDeleteAt {
    pub id: i32,
    pub user_id: i32,
    pub card_number: String,
    pub card_type: String,
    pub expire_date: String,
    pub cvv: String,
    pub card_provider: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct DashboardCard {
    pub total_balance: Option<i64>,
    pub total_topup: Option<i64>,
    pub total_withdraw: Option<i64>,
    pub total_transaction: Option<i64>,
    pub total_transfer: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct DashboardCardCardNumber {
    pub total_balance: Option<i64>,
    pub total_topup: Option<i64>,
    pub total_withdraw: Option<i64>,
    pub total_transaction: Option<i64>,
    pub total_transfer_send: Option<i64>,
    pub total_transfer_receiver: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CardResponseMonthBalance {
    pub month: String,
    pub total_balance: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CardResponseYearlyBalance {
    pub year: String,
    pub total_balance: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CardResponseMonthAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CardResponseYearAmount {
    pub year: String,
    pub total_amount: i64,
}

