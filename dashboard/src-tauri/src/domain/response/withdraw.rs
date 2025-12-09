use crate::model::withdraw::{
    WithdrawMonthlyAmountResponse, WithdrawResponse, WithdrawResponseDeleteAt,
    WithdrawResponseMonthStatusFailed, WithdrawResponseMonthStatusSuccess,
    WithdrawResponseYearStatusFailed, WithdrawResponseYearStatusSuccess,
    WithdrawYearlyAmountResponse,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawMonthStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponseMonthStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawYearStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponseYearStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawMonthStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponseMonthStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawYearStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponseYearStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawMonthAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawMonthlyAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawYearAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawYearlyAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsesWithdraw {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdraw {
    pub status: String,
    pub message: String,
    pub data: Option<WithdrawResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithdrawAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponsePaginationWithdraw {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationWithdrawDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<WithdrawResponseDeleteAt>,
    pub pagination: PaginationMeta,
}
