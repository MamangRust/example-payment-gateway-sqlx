use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct CreateUserRoleRequest {
    #[validate(range(min = 1, message = "User ID wajib diisi"))]
    pub user_id: i32,

    #[validate(range(min = 1, message = "Role ID wajib diisi"))]
    pub role_id: i32,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct RemoveUserRoleRequest {
    #[validate(range(min = 1, message = "User ID wajib diisi"))]
    pub user_id: i32,

    #[validate(range(min = 1, message = "Role ID wajib diisi"))]
    pub role_id: i32,
}
