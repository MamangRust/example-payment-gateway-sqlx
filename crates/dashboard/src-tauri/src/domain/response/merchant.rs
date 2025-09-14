use crate::model::merchant::{
    MerchantMonthlyTotalAmount, MerchantResponse, MerchantResponseDeleteAt,
    MerchantResponseMonthlyAmount, MerchantResponseMonthlyPaymentMethod,
    MerchantResponseYearlyAmount, MerchantResponseYearlyPaymentMethod, MerchantTransactionResponse,
    MerchantYearlyTotalAmount,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantMonthlyPaymentMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponseMonthlyPaymentMethod>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantYearlyPaymentMethod {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponseYearlyPaymentMethod>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantMonthlyAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponseMonthlyAmount>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantYearlyAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponseYearlyAmount>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsesMerchant {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchant {
    pub status: String,
    pub message: String,
    pub data: MerchantResponse,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMerchantAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationMerchant {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationMerchantDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantResponseDeleteAt>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationMerchantTransaction {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantTransactionResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseMerchantMonthlyTotalAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantMonthlyTotalAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseMerchantYearlyTotalAmount {
    pub status: String,
    pub message: String,
    pub data: Vec<MerchantYearlyTotalAmount>,
}
