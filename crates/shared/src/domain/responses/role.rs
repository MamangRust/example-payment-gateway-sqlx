use crate::{model::role::RoleModel, utils::parse_datetime};
use genproto::role::{
    RoleResponse as RoleResponseProto, RoleResponseDeleteAt as RoleResponseDeleteAtProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RoleResponseDeleteAt {
    pub id: i32,
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

// model to response
impl From<RoleModel> for RoleResponse {
    fn from(value: RoleModel) -> Self {
        RoleResponse {
            id: value.role_id,
            name: value.role_name,
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<RoleModel> for RoleResponseDeleteAt {
    fn from(value: RoleModel) -> Self {
        RoleResponseDeleteAt {
            id: value.role_id,
            name: value.role_name,
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
            deleted_at: value.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

// response to proto
impl From<RoleResponse> for RoleResponseProto {
    fn from(value: RoleResponse) -> Self {
        RoleResponseProto {
            id: value.id,
            name: value.name,
            created_at: value.created_at.unwrap_or_default(),
            updated_at: value.updated_at.unwrap_or_default(),
        }
    }
}

impl From<RoleResponseDeleteAt> for RoleResponseDeleteAtProto {
    fn from(value: RoleResponseDeleteAt) -> Self {
        RoleResponseDeleteAtProto {
            id: value.id,
            name: value.name,
            created_at: value.created_at.unwrap_or_default(),
            updated_at: value.updated_at.unwrap_or_default(),
            deleted_at: Some(value.deleted_at.unwrap_or_default()),
        }
    }
}

// proto to response
impl From<RoleResponseProto> for RoleResponse {
    fn from(value: RoleResponseProto) -> Self {
        RoleResponse {
            id: value.id,
            name: value.name,
            created_at: parse_datetime(&value.created_at),
            updated_at: parse_datetime(&value.updated_at),
        }
    }
}

impl From<RoleResponseDeleteAtProto> for RoleResponseDeleteAt {
    fn from(value: RoleResponseDeleteAtProto) -> Self {
        RoleResponseDeleteAt {
            id: value.id,
            name: value.name,
            created_at: parse_datetime(&value.created_at),
            updated_at: parse_datetime(&value.updated_at),
            deleted_at: value.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}
