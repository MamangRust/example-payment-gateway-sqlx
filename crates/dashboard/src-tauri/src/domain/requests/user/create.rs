use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
