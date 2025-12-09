use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMerchant {
    pub merchant_id: u32,
    pub name: String,
    pub user_id: u32,
    pub status: String,
}
