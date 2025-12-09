use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseDeleteAt {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}
