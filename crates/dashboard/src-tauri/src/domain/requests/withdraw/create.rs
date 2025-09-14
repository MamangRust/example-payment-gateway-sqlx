use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWithdraw {
    pub card_number: String,
    pub withdraw_amount: f64,
    pub withdraw_time: DateTime<Utc>,
}
