use crate::model::user::{UserResponse, UserResponseDeleteAt};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseUser {
    pub status: String,
    pub message: String,
    pub data: Option<UserResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsesUser {
    pub status: String,
    pub message: String,
    pub data: Vec<UserResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseUserDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseUserAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponsePaginationUserDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<UserResponseDeleteAt>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationUser {
    pub status: String,
    pub message: String,
    pub data: Vec<UserResponse>,
    pub pagination: PaginationMeta,
}
