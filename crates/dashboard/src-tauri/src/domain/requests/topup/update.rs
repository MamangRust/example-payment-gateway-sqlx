use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTopup {
    pub id: u32,
    pub card_number: String,
    pub topup_amount: f64,
    pub topup_method: String,
}
