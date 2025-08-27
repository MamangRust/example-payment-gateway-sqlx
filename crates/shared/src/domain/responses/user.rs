use crate::{model::user::UserModel, utils::parse_datetime};
use genproto::user::{
    UserResponse as UserResponseProto, UserResponseDeleteAt as UserResponseDeleteAtProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDeleteAt {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

// model to response
impl From<UserModel> for UserResponse {
    fn from(value: UserModel) -> Self {
        UserResponse {
            id: value.id,
            firstname: value.first_name,
            lastname: value.last_name,
            email: value.email,
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<UserModel> for UserResponseDeleteAt {
    fn from(value: UserModel) -> Self {
        UserResponseDeleteAt {
            id: value.id,
            firstname: value.first_name,
            lastname: value.last_name,
            email: value.email,
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
            deleted_at: value.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

pub fn to_user_response(opt: Option<UserModel>) -> Option<UserResponse> {
    opt.map(UserResponse::from)
}

// response to proto
impl From<UserResponse> for UserResponseProto {
    fn from(value: UserResponse) -> Self {
        UserResponseProto {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            created_at: value.created_at.unwrap_or_default(),
            updated_at: value.updated_at.unwrap_or_default(),
        }
    }
}

impl From<UserResponseDeleteAt> for UserResponseDeleteAtProto {
    fn from(value: UserResponseDeleteAt) -> Self {
        UserResponseDeleteAtProto {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            created_at: value.created_at.unwrap_or_default(),
            updated_at: value.updated_at.unwrap_or_default(),
            deleted_at: Some(value.deleted_at.unwrap_or_default()),
        }
    }
}

// proto to response
impl From<UserResponseProto> for UserResponse {
    fn from(value: UserResponseProto) -> Self {
        UserResponse {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            created_at: parse_datetime(&value.created_at),
            updated_at: parse_datetime(&value.updated_at),
        }
    }
}
impl From<UserResponseDeleteAtProto> for UserResponseDeleteAt {
    fn from(value: UserResponseDeleteAtProto) -> Self {
        UserResponseDeleteAt {
            id: value.id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            created_at: parse_datetime(&value.created_at),
            updated_at: parse_datetime(&value.updated_at),
            deleted_at: value.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}
