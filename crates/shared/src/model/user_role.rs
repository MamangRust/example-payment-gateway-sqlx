use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserRoleModel {
    #[serde(rename = "user_role_id")]
    pub user_role_id: i32,
    #[serde(rename = "user_id")]
    pub user_id: i32,
    #[serde(rename = "role_id")]
    pub role_id: i32,
    #[serde(rename = "role_name", skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
