use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePermanentMerchant {
    pub id: u32,
}
