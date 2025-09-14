use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByCardNumberSaldo {
    pub card_number: String,
}
