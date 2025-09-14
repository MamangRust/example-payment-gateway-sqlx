use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSaldo {
    pub card_number: String,
    pub total_balance: f64,
}
