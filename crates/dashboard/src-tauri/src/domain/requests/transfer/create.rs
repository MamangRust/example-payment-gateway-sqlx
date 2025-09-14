use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTransfer {
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: f64,
}
