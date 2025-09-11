use crate::{
    model::saldo::{
        SaldoModel, SaldoMonthSaldoBalance, SaldoMonthTotalBalance, SaldoYearSaldoBalance,
        SaldoYearTotalBalance,
    },
    utils::parse_datetime,
};
use genproto::saldo::{
    SaldoMonthBalanceResponse as SaldoMonthBalanceResponseProto,
    SaldoMonthTotalBalanceResponse as SaldoMonthTotalBalanceResponseProto,
    SaldoResponse as SaldoResponseProto, SaldoResponseDeleteAt as SaldoResponseDeleteAtProto,
    SaldoYearBalanceResponse as SaldoYearBalanceResponseProto,
    SaldoYearTotalBalanceResponse as SaldoYearTotalBalanceResponseProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoResponse {
    pub id: i32,
    pub card_number: String,
    pub total_balance: i64,
    pub withdraw_amount: i32,
    pub withdraw_time: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoResponseDeleteAt {
    pub id: i32,
    pub card_number: String,
    pub total_balance: i64,
    pub withdraw_amount: i32,
    pub withdraw_time: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoMonthTotalBalanceResponse {
    pub month: String,
    pub year: String,
    pub total_balance: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoYearTotalBalanceResponse {
    pub year: String,
    pub total_balance: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoMonthBalanceResponse {
    pub month: String,
    pub total_balance: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaldoYearBalanceResponse {
    pub year: String,
    pub total_balance: i64,
}

// model to response
impl From<SaldoModel> for SaldoResponse {
    fn from(model: SaldoModel) -> Self {
        Self {
            id: model.saldo_id,
            card_number: model.card_number,
            total_balance: model.total_balance,
            withdraw_amount: model.withdraw_amount.unwrap_or(0),
            withdraw_time: model.withdraw_time.map(|dt| dt.to_string()),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<SaldoModel> for SaldoResponseDeleteAt {
    fn from(model: SaldoModel) -> Self {
        Self {
            id: model.saldo_id,
            card_number: model.card_number,
            total_balance: model.total_balance,
            withdraw_amount: model.withdraw_amount.unwrap_or(0),
            withdraw_time: model.withdraw_time.map(|dt| dt.to_string()),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<SaldoMonthTotalBalance> for SaldoMonthTotalBalanceResponse {
    fn from(m: SaldoMonthTotalBalance) -> Self {
        Self {
            month: m.month,
            year: m.year,
            total_balance: m.total_balance,
        }
    }
}

impl From<SaldoYearTotalBalance> for SaldoYearTotalBalanceResponse {
    fn from(y: SaldoYearTotalBalance) -> Self {
        Self {
            year: y.year,
            total_balance: y.total_balance,
        }
    }
}

impl From<SaldoMonthSaldoBalance> for SaldoMonthBalanceResponse {
    fn from(m: SaldoMonthSaldoBalance) -> Self {
        Self {
            month: m.month,
            total_balance: m.total_balance,
        }
    }
}

impl From<SaldoYearSaldoBalance> for SaldoYearBalanceResponse {
    fn from(y: SaldoYearSaldoBalance) -> Self {
        Self {
            year: y.year,
            total_balance: y.total_balance,
        }
    }
}

// response to proto
impl From<SaldoResponse> for SaldoResponseProto {
    fn from(r: SaldoResponse) -> Self {
        Self {
            saldo_id: r.id,
            card_number: r.card_number,
            total_balance: r.total_balance,
            withdraw_amount: r.withdraw_amount,
            withdraw_time: r.withdraw_time.unwrap_or_default(),
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<SaldoResponseDeleteAt> for SaldoResponseDeleteAtProto {
    fn from(r: SaldoResponseDeleteAt) -> Self {
        Self {
            saldo_id: r.id,
            card_number: r.card_number,
            total_balance: r.total_balance,
            withdraw_amount: r.withdraw_amount,
            withdraw_time: r.withdraw_time.unwrap_or_default(),
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<SaldoMonthTotalBalanceResponse> for SaldoMonthTotalBalanceResponseProto {
    fn from(r: SaldoMonthTotalBalanceResponse) -> Self {
        Self {
            month: r.month,
            year: r.year,
            total_balance: r.total_balance,
        }
    }
}

impl From<SaldoYearTotalBalanceResponse> for SaldoYearTotalBalanceResponseProto {
    fn from(r: SaldoYearTotalBalanceResponse) -> Self {
        Self {
            year: r.year,
            total_balance: r.total_balance,
        }
    }
}

impl From<SaldoMonthBalanceResponse> for SaldoMonthBalanceResponseProto {
    fn from(r: SaldoMonthBalanceResponse) -> Self {
        Self {
            month: r.month,
            total_balance: r.total_balance,
        }
    }
}

impl From<SaldoYearBalanceResponse> for SaldoYearBalanceResponseProto {
    fn from(r: SaldoYearBalanceResponse) -> Self {
        Self {
            year: r.year,
            total_balance: r.total_balance,
        }
    }
}

// proto to response
impl From<SaldoResponseProto> for SaldoResponse {
    fn from(p: SaldoResponseProto) -> Self {
        Self {
            id: p.saldo_id,
            card_number: p.card_number,
            total_balance: p.total_balance,
            withdraw_amount: p.withdraw_amount,
            withdraw_time: parse_datetime(&p.withdraw_time),
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<SaldoResponseDeleteAtProto> for SaldoResponseDeleteAt {
    fn from(p: SaldoResponseDeleteAtProto) -> Self {
        Self {
            id: p.saldo_id,
            card_number: p.card_number,
            total_balance: p.total_balance,
            withdraw_amount: p.withdraw_amount,
            withdraw_time: parse_datetime(&p.withdraw_time),
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<SaldoMonthTotalBalanceResponseProto> for SaldoMonthTotalBalanceResponse {
    fn from(p: SaldoMonthTotalBalanceResponseProto) -> Self {
        Self {
            month: p.month,
            year: p.year,
            total_balance: p.total_balance,
        }
    }
}

impl From<SaldoYearTotalBalanceResponseProto> for SaldoYearTotalBalanceResponse {
    fn from(p: SaldoYearTotalBalanceResponseProto) -> Self {
        Self {
            year: p.year,
            total_balance: p.total_balance,
        }
    }
}

impl From<SaldoMonthBalanceResponseProto> for SaldoMonthBalanceResponse {
    fn from(p: SaldoMonthBalanceResponseProto) -> Self {
        Self {
            month: p.month,
            total_balance: p.total_balance,
        }
    }
}

impl From<SaldoYearBalanceResponseProto> for SaldoYearBalanceResponse {
    fn from(p: SaldoYearBalanceResponseProto) -> Self {
        Self {
            year: p.year,
            total_balance: p.total_balance,
        }
    }
}
