use crate::model::transfer::{
    TransferMonthAmountResponse, TransferResponse, TransferResponseDeleteAt,
    TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
    TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
    TransferYearAmountResponse,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferMonthStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponseMonthStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferYearStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponseYearStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferMonthStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponseMonthStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferYearStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponseYearStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferMonthAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferMonthAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferYearAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferYearAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransfer {
    pub status: String,
    pub message: String,
    pub data: Option<TransferResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransfers {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransferAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTransfer {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponse>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTransferDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<TransferResponseDeleteAt>,
    pub pagination: Option<PaginationMeta>,
}
