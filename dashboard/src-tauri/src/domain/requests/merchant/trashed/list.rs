use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllMerchantTrashed {
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}
