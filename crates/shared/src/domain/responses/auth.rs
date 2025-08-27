use genproto::auth::TokenResponse as ProtoTokenResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<ProtoTokenResponse> for TokenResponse {
    fn from(value: ProtoTokenResponse) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}

impl From<TokenResponse> for ProtoTokenResponse {
    fn from(value: TokenResponse) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}
