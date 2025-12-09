use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams, Clone)]
pub struct FindAllRoles {
    #[serde(default = "default_page")]
    pub page: i32,

    #[serde(default = "default_page_size")]
    pub page_size: i32,

    #[serde(default)]
    pub search: String,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, message = "Nama role wajib diisi"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct UpdateRoleRequest {
    pub id: Option<i32>,

    #[validate(length(min = 1, message = "Nama role wajib diisi"))]
    pub name: String,
}
