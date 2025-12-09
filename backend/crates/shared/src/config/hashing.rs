use crate::{abstract_trait::hashing::HashingTrait, errors::ServiceError};
use async_trait::async_trait;
use bcrypt::{BcryptError, hash, verify};

#[derive(Clone)]
pub struct Hashing;

impl Hashing {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Hashing {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HashingTrait for Hashing {
    async fn hash_password(&self, password: &str) -> Result<String, ServiceError> {
        let hashed = hash(password, 4).map_err(ServiceError::Bcrypt)?;
        Ok(hashed)
    }

    async fn compare_password(
        &self,
        hashed_password: &str,
        password: &str,
    ) -> Result<(), ServiceError> {
        verify(password, hashed_password)
            .map_err(|e| match e {
                BcryptError::InvalidHash(_) => ServiceError::Bcrypt(e),
                _ => ServiceError::Bcrypt(e),
            })
            .and_then(|is_valid| {
                if is_valid {
                    Ok(())
                } else {
                    Err(ServiceError::InvalidCredentials)
                }
            })
    }
}
