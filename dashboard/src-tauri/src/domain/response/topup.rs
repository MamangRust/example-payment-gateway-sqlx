use crate::model::topup::{
    TopupMonthAmountResponse, TopupMonthMethodResponse, TopupResponse, TopupResponseDeleteAt,
    TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess, TopupResponseYearStatusFailed,
    TopupResponseYearStatusSuccess, TopupYearlyAmountResponse, TopupYearlyMethodResponse,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupMonthStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponseMonthStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupYearStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponseYearStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupMonthStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponseMonthStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupYearStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponseYearStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupMonthMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupMonthMethodResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupYearMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupYearlyMethodResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupMonthAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupMonthAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupYearAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupYearlyAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopup {
    pub status: String,
    pub message: String,
    pub data: Option<TopupResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsesTopup {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTopup {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponse>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTopupDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<TopupResponseDeleteAt>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTopupAll {
    pub status: String,
    pub message: String,
}
