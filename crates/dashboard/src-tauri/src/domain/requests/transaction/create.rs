use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTransaction {
    pub card_number: String,
    pub amount: f64,
    pub payment_method: String,
    pub merchant_id: u32,
    pub api_key: String,
    pub transaction_time: DateTime<Utc>,
}
