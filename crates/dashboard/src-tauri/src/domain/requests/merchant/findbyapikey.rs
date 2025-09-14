use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByApiKeyMerchant {
    pub api_key: String,
}
