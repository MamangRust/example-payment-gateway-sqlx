use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByCardNumberWithdraw {
    pub card_number: String,
}
