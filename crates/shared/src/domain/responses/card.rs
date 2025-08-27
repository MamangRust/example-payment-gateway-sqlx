use crate::{
    model::card::{
        CardModel, CardMonthAmount, CardMonthBalance, CardYearAmount, CardYearlyBalance,
    },
    utils::parse_datetime,
};
use utoipa::ToSchema;

use genproto::card::{
    CardResponse as CardResponseProto, CardResponseDashboard as CardResponseDashboardProto,
    CardResponseDashboardCardNumber as CardResponseDashboardCardNumberProto,
    CardResponseDeleteAt as CardResponseDeleteAtProto,
    CardResponseMonthlyAmount as CardResponseMonthlyAmountProto,
    CardResponseMonthlyBalance as CardResponseMonthBalanceProto,
    CardResponseYearlyAmount as CardResponseYearAmountProto,
    CardResponseYearlyBalance as CardResponseYearBalanceProto,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponse {
    pub id: i32,
    #[serde(rename = "user_id")]
    pub user_id: i32,
    #[serde(rename = "card_number")]
    pub card_number: String,
    #[serde(rename = "card_type")]
    pub card_type: String,
    #[serde(rename = "expire_date")]
    pub expire_date: String,
    pub cvv: String,
    #[serde(rename = "card_provider")]
    pub card_provider: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponseDeleteAt {
    pub id: i32,
    #[serde(rename = "user_id")]
    pub user_id: i32,
    #[serde(rename = "card_number")]
    pub card_number: String,
    #[serde(rename = "card_type")]
    pub card_type: String,
    #[serde(rename = "expire_date")]
    pub expire_date: String,
    pub cvv: String,
    #[serde(rename = "card_provider")]
    pub card_provider: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DashboardCard {
    #[serde(rename = "total_balance")]
    pub total_balance: Option<i64>,
    #[serde(rename = "total_topup")]
    pub total_topup: Option<i64>,
    #[serde(rename = "total_withdraw")]
    pub total_withdraw: Option<i64>,
    #[serde(rename = "total_transaction")]
    pub total_transaction: Option<i64>,
    #[serde(rename = "total_transfer")]
    pub total_transfer: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DashboardCardCardNumber {
    #[serde(rename = "total_balance")]
    pub total_balance: Option<i64>,
    #[serde(rename = "total_topup")]
    pub total_topup: Option<i64>,
    #[serde(rename = "total_withdraw")]
    pub total_withdraw: Option<i64>,
    #[serde(rename = "total_transaction")]
    pub total_transaction: Option<i64>,
    #[serde(rename = "total_transfer_send")]
    pub total_transfer_send: Option<i64>,
    #[serde(rename = "total_transfer_receiver")]
    pub total_transfer_receiver: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponseMonthBalance {
    pub month: String,
    #[serde(rename = "total_balance")]
    pub total_balance: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponseYearlyBalance {
    pub year: String,
    #[serde(rename = "total_balance")]
    pub total_balance: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponseMonthAmount {
    pub month: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CardResponseYearAmount {
    pub year: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
}

// model to response
impl From<CardModel> for CardResponse {
    fn from(model: CardModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            card_number: model.card_number,
            card_type: model.card_type,
            expire_date: model.expire_date,
            cvv: model.cvv,
            card_provider: model.card_provider,
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<CardModel> for CardResponseDeleteAt {
    fn from(model: CardModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            card_number: model.card_number,
            card_type: model.card_type,
            expire_date: model.expire_date,
            cvv: model.cvv,
            card_provider: model.card_provider,
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<CardMonthBalance> for CardResponseMonthBalance {
    fn from(b: CardMonthBalance) -> Self {
        Self {
            month: b.month,
            total_balance: b.total_balance,
        }
    }
}

impl From<CardYearlyBalance> for CardResponseYearlyBalance {
    fn from(b: CardYearlyBalance) -> Self {
        Self {
            year: b.year,
            total_balance: b.total_balance,
        }
    }
}

impl From<CardMonthAmount> for CardResponseMonthAmount {
    fn from(a: CardMonthAmount) -> Self {
        Self {
            month: a.month,
            total_amount: a.total_amount,
        }
    }
}

impl From<CardYearAmount> for CardResponseYearAmount {
    fn from(a: CardYearAmount) -> Self {
        Self {
            year: a.year,
            total_amount: a.total_amount,
        }
    }
}

// response to proto
impl From<DashboardCard> for CardResponseDashboardProto {
    fn from(d: DashboardCard) -> Self {
        Self {
            total_balance: d.total_balance.unwrap_or(0),
            total_topup: d.total_topup.unwrap_or(0),
            total_withdraw: d.total_withdraw.unwrap_or(0),
            total_transaction: d.total_transaction.unwrap_or(0),
            total_transfer: d.total_transfer.unwrap_or(0),
        }
    }
}

impl From<DashboardCardCardNumber> for CardResponseDashboardCardNumberProto {
    fn from(d: DashboardCardCardNumber) -> Self {
        Self {
            total_balance: d.total_balance.unwrap_or(0),
            total_topup: d.total_topup.unwrap_or(0),
            total_withdraw: d.total_withdraw.unwrap_or(0),
            total_transaction: d.total_transaction.unwrap_or(0),
            total_transfer_send: d.total_transfer_send.unwrap_or(0),
            total_transfer_receiver: d.total_transfer_receiver.unwrap_or(0),
        }
    }
}

impl From<CardResponse> for CardResponseProto {
    fn from(r: CardResponse) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            card_number: r.card_number,
            card_type: r.card_type,
            expire_date: r.expire_date,
            cvv: r.cvv,
            card_provider: r.card_provider,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<CardResponseDeleteAt> for CardResponseDeleteAtProto {
    fn from(r: CardResponseDeleteAt) -> Self {
        Self {
            id: r.id,
            user_id: r.user_id,
            card_number: r.card_number,
            card_type: r.card_type,
            expire_date: r.expire_date,
            cvv: r.cvv,
            card_provider: r.card_provider,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<CardResponseMonthBalance> for CardResponseMonthBalanceProto {
    fn from(r: CardResponseMonthBalance) -> Self {
        Self {
            month: r.month,
            total_balance: r.total_balance,
        }
    }
}

impl From<CardResponseYearlyBalance> for CardResponseYearBalanceProto {
    fn from(r: CardResponseYearlyBalance) -> Self {
        Self {
            year: r.year,
            total_balance: r.total_balance,
        }
    }
}

impl From<CardResponseMonthAmount> for CardResponseMonthlyAmountProto {
    fn from(r: CardResponseMonthAmount) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount,
        }
    }
}

impl From<CardResponseYearAmount> for CardResponseYearAmountProto {
    fn from(r: CardResponseYearAmount) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
        }
    }
}

// proto to response
impl From<CardResponseDashboardProto> for DashboardCard {
    fn from(proto: CardResponseDashboardProto) -> Self {
        Self {
            total_balance: Some(proto.total_balance),
            total_topup: Some(proto.total_topup),
            total_withdraw: Some(proto.total_withdraw),
            total_transaction: Some(proto.total_transaction),
            total_transfer: Some(proto.total_transfer),
        }
    }
}

impl From<CardResponseDashboardCardNumberProto> for DashboardCardCardNumber {
    fn from(proto: CardResponseDashboardCardNumberProto) -> Self {
        Self {
            total_balance: Some(proto.total_balance),
            total_topup: Some(proto.total_topup),
            total_withdraw: Some(proto.total_withdraw),
            total_transaction: Some(proto.total_transaction),
            total_transfer_send: Some(proto.total_transfer_send),
            total_transfer_receiver: Some(proto.total_transfer_receiver),
        }
    }
}

impl From<CardResponseProto> for CardResponse {
    fn from(p: CardResponseProto) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            card_number: p.card_number,
            card_type: p.card_type,
            expire_date: p.expire_date,
            cvv: p.cvv,
            card_provider: p.card_provider,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<CardResponseDeleteAtProto> for CardResponseDeleteAt {
    fn from(p: CardResponseDeleteAtProto) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            card_number: p.card_number,
            card_type: p.card_type,
            expire_date: p.expire_date,
            cvv: p.cvv,
            card_provider: p.card_provider,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<CardResponseMonthBalanceProto> for CardResponseMonthBalance {
    fn from(p: CardResponseMonthBalanceProto) -> Self {
        Self {
            month: p.month,
            total_balance: p.total_balance,
        }
    }
}

impl From<CardResponseYearBalanceProto> for CardResponseYearlyBalance {
    fn from(p: CardResponseYearBalanceProto) -> Self {
        Self {
            year: p.year,
            total_balance: p.total_balance,
        }
    }
}

impl From<CardResponseMonthlyAmountProto> for CardResponseMonthAmount {
    fn from(p: CardResponseMonthlyAmountProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<CardResponseYearAmountProto> for CardResponseYearAmount {
    fn from(p: CardResponseYearAmountProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
