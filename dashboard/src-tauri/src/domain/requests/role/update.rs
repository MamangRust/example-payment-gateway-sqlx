use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRole {
    pub role_id: i32,
    pub name: String,
}
