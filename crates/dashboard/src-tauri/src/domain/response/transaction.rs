use crate::model::transaction::{
    TransactionMonthAmountResponse, TransactionMonthMethodResponse, TransactionResponse,
    TransactionResponseDeleteAt, TransactionResponseMonthStatusFailed,
    TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
    TransactionResponseYearStatusSuccess, TransactionYearMethodResponse,
    TransactionYearlyAmountResponse,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionMonthStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponseMonthStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionYearStatusSuccess {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponseYearStatusSuccess>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionMonthStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponseMonthStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionYearStatusFailed {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponseYearStatusFailed>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionMonthMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionMonthMethodResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionYearMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionYearMethodResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionMonthAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionMonthAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionYearAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionYearlyAmountResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransaction {
    pub status: String,
    pub message: String,
    pub data: Option<TransactionResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactions {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseTransactionAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTransaction {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponse>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationTransactionDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<TransactionResponseDeleteAt>,
    pub pagination: Option<PaginationMeta>,
}
