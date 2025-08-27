use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: i32,
    #[serde(rename = "firstname")]
    pub first_name: String,
    #[serde(rename = "lastname")]
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "confirm_password")]
    pub confirm_password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
