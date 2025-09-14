use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByCardNumberTopup {
    pub card_number: String,
}
