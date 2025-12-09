use serde::{Deserialize, Serialize};

use crate::model::card::{
    CardResponse, CardResponseDeleteAt, CardResponseMonthAmount, CardResponseMonthBalance,
    CardResponseYearAmount, CardResponseYearlyBalance, DashboardCard, DashboardCardCardNumber,
};

use super::PaginationMeta;

#[derive(Serialize, Deserialize)]
pub struct ApiResponseCard {
    pub status: String,
    pub message: String,
    pub data: Option<CardResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseCardDelete {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseCardAll {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationCard {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponse>>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponsePaginationCardDeleteAt {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponseDeleteAt>>,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMonthlyBalance {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponseMonthBalance>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseYearlyBalance {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponseYearlyBalance>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseMonthlyAmount {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponseMonthAmount>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseYearlyAmount {
    pub status: String,
    pub message: String,
    pub data: Option<Vec<CardResponseYearAmount>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseDashboardCard {
    pub status: String,
    pub message: String,
    pub data: Option<DashboardCard>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponseDashboardCardNumber {
    pub status: String,
    pub message: String,
    pub data: Option<DashboardCardCardNumber>,
}
