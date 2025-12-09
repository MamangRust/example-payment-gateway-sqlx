use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMerchant {
    pub name: String,
    pub user_id: u32,
}
