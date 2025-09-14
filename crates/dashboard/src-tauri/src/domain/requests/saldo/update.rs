use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSaldo {
    pub id: u32,
    pub card_number: String,
    pub total_balance: f64,
}
