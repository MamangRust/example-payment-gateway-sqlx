use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
