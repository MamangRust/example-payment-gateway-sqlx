use crate::domain::responses::pagination::Pagination;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T: std::fmt::Debug> fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiResponse {{ status: {}, message: {}, data: {:?} }}",
            self.status, self.message, self.data
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ApiResponsePagination<T> {
    pub status: String,
    pub message: String,
    pub data: T,
    pub pagination: Pagination,
}

impl<T: Serialize> fmt::Display for ApiResponsePagination<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "{json}"),
            Err(e) => write!(f, "Error serializing ApiResponse to JSON: {e}"),
        }
    }
}
