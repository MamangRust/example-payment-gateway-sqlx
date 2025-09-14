use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateCard {
    pub card_id: u32,
    pub user_id: u32,
    pub card_type: String,
    pub expire_date: String,
    pub cvv: String,
    pub card_provider: String,
}
