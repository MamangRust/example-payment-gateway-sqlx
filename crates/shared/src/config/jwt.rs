use crate::{abstract_trait::jwt::JwtServiceTrait, errors::ServiceError};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

impl Claims {
    pub fn new(user_id: i64, exp: usize, iat: usize, token_type: String) -> Self {
        Claims {
            user_id,
            exp,
            iat,
            token_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub jwt_secret: String,
}

impl JwtConfig {
    pub fn new(jwt_secret: &str) -> Self {
        JwtConfig {
            jwt_secret: jwt_secret.to_string(),
        }
    }
}

#[async_trait]
impl JwtServiceTrait for JwtConfig {
    fn generate_token(&self, user_id: i64, token_type: &str) -> Result<String, ServiceError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = match token_type {
            "access" => (now + Duration::minutes(60)).timestamp() as usize,
            "refresh" => (now + Duration::days(7)).timestamp() as usize,
            _ => return Err(ServiceError::InvalidTokenType),
        };

        let claims = Claims::new(user_id, exp, iat, token_type.to_string());

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(ServiceError::Jwt)
    }

    fn verify_token(&self, token: &str, expected_type: &str) -> Result<i64, ServiceError> {
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_ref());
        let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
            .map_err(ServiceError::Jwt)?;

        let current_time = Utc::now().timestamp() as usize;

        if token_data.claims.exp < current_time {
            return Err(ServiceError::TokenExpired);
        }

        if token_data.claims.token_type != expected_type {
            return Err(ServiceError::InvalidTokenType);
        }

        Ok(token_data.claims.user_id)
    }
}
