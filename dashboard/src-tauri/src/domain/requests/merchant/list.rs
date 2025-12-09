use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllMerchant {
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllMerchantTransaction {
    pub merchant_id: u32,
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllMerchantTransactionApiKey {
    pub api_key: String,
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}
