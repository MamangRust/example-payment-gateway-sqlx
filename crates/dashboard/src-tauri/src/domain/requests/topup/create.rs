use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTopup {
    pub card_number: String,
    pub topup_amount: f64,
    pub topup_method: String,
}
