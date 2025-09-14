use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}
