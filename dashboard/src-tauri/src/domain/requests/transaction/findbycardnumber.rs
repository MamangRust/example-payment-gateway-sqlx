use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByCardNumberTransaction {
    pub card_number: String,
}
