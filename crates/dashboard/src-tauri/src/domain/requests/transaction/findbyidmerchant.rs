use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByMerchantTransaction {
    pub id: u32,
}
