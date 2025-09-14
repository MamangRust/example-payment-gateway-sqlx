use crate::model::saldo::{
    SaldoMonthBalanceResponse, SaldoMonthTotalBalanceResponse, SaldoResponse,
    SaldoResponseDeleteAt, SaldoYearBalanceResponse, SaldoYearTotalBalanceResponse,
};

use super::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponseSaldo {
    pub status: String,
    pub message: String,
    pub data: SaldoResponse,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsesSaldo {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseSaldoDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseSaldoAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMonthTotalSaldo {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoMonthTotalBalanceResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseYearTotalSaldo {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoYearTotalBalanceResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMonthSaldoBalances {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoMonthBalanceResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseYearSaldoBalances {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoYearBalanceResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponsePaginationSaldo {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationSaldoDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Vec<SaldoResponseDeleteAt>,
    pub pagination: PaginationMeta,
}
