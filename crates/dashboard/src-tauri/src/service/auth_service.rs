use reqwest::{Client, Error};

use crate::{
    domain::requests::auth::{
        login::LoginRequest, refresh_token::RefreshTokenRequest, register::RegisterRequest,
    },
    helpers::api::create_client,
    model::auth::{
        ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister,
    },
};

pub struct AuthService {
    client: Client,
    base_url: String,
}

impl AuthService {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: create_client(),
        }
    }

    pub async fn login(&self, req: &LoginRequest) -> Result<ApiResponseLogin, Error> {
        let response = self
            .client
            .post(format!("{}/auth/login", self.base_url))
            .json(req)
            .send()
            .await?
            .json::<ApiResponseLogin>()
            .await?;

        Ok(response)
    }

    pub async fn register(&self, req: &RegisterRequest) -> Result<ApiResponseRegister, Error> {
        let response = self
            .client
            .post(format!("{}/auth/register", self.base_url))
            .json(req)
            .send()
            .await?
            .json::<ApiResponseRegister>()
            .await?;

        Ok(response)
    }

    pub async fn get_me(&self, access_token: &str) -> Result<ApiResponseGetMe, Error> {
        let response = self
            .client
            .get(format!("{}/auth/me", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseGetMe>()
            .await?;

        Ok(response)
    }

    pub async fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<ApiResponseRefreshToken, Error> {
        let response = self
            .client
            .post(format!("{}/auth/refresh-token", self.base_url))
            .json(&RefreshTokenRequest {
                refresh_token: refresh_token.to_string(),
            })
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<ApiResponseRefreshToken>()
            .await?;

        Ok(response)
    }
}
