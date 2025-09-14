use crate::{
    model::transaction::{
        TransactionModel, TransactionModelMonthStatusFailed, TransactionModelMonthStatusSuccess,
        TransactionModelYearStatusFailed, TransactionModelYearStatusSuccess,
        TransactionMonthAmount, TransactionMonthMethod, TransactionYearMethod,
        TransactionYearlyAmount,
    },
    utils::parse_datetime,
};
use genproto::transaction::{
    TransactionMonthAmountResponse as TransactionMonthAmountResponseProto,
    TransactionMonthMethodResponse as TransactionMonthMethodResponseProto,
    TransactionMonthStatusFailedResponse as TransactionResponseMonthStatusFailedProto,
    TransactionMonthStatusSuccessResponse as TransactionResponseMonthStatusSuccessProto,
    TransactionResponse as TransactionResponseProto,
    TransactionResponseDeleteAt as TransactionResponseDeleteAtProto,
    TransactionYearMethodResponse as TransactionYearMethodResponseProto,
    TransactionYearStatusFailedResponse as TransactionResponseYearStatusFailedProto,
    TransactionYearStatusSuccessResponse as TransactionResponseYearStatusSuccessProto,
    TransactionYearlyAmountResponse as TransactionYearlyAmountResponseProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub id: i32,
    pub transaction_no: String,
    pub card_number: String,
    pub amount: i64,
    pub payment_method: String,
    pub merchant_id: i32,
    pub transaction_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponseDeleteAt {
    pub id: i32,
    pub transaction_no: String,
    pub card_number: String,
    pub amount: i64,
    pub payment_method: String,
    pub merchant_id: i32,
    pub transaction_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponseMonthStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub month: String,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionMonthMethodResponse {
    pub month: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionYearMethodResponse {
    pub year: String,
    pub payment_method: String,
    pub total_transactions: i32,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionMonthAmountResponse {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionYearlyAmountResponse {
    pub year: String,
    pub total_amount: i64,
}

// model to response
impl From<TransactionModel> for TransactionResponse {
    fn from(model: TransactionModel) -> Self {
        Self {
            id: model.transaction_id,
            transaction_no: model.transaction_no.to_string(),
            card_number: model.card_number,
            amount: model.amount as i64,
            payment_method: model.payment_method,
            merchant_id: model.merchant_id,
            transaction_time: model.transaction_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TransactionModel> for TransactionResponseDeleteAt {
    fn from(model: TransactionModel) -> Self {
        Self {
            id: model.transaction_id,
            transaction_no: model.transaction_no.to_string(),
            card_number: model.card_number,
            amount: model.amount as i64,
            payment_method: model.payment_method,
            merchant_id: model.merchant_id,
            transaction_time: model.transaction_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TransactionModelMonthStatusSuccess> for TransactionResponseMonthStatusSuccess {
    fn from(m: TransactionModelMonthStatusSuccess) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_success: m.total_success,
        }
    }
}

impl From<TransactionModelYearStatusSuccess> for TransactionResponseYearStatusSuccess {
    fn from(y: TransactionModelYearStatusSuccess) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_success: y.total_success,
        }
    }
}

impl From<TransactionModelMonthStatusFailed> for TransactionResponseMonthStatusFailed {
    fn from(m: TransactionModelMonthStatusFailed) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_failed: m.total_failed,
        }
    }
}

impl From<TransactionModelYearStatusFailed> for TransactionResponseYearStatusFailed {
    fn from(y: TransactionModelYearStatusFailed) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_failed: y.total_failed,
        }
    }
}

impl From<TransactionMonthMethod> for TransactionMonthMethodResponse {
    fn from(m: TransactionMonthMethod) -> Self {
        Self {
            month: m.month,
            payment_method: m.payment_method,
            total_transactions: m.total_transactions,
            total_amount: m.total_amount,
        }
    }
}

impl From<TransactionYearMethod> for TransactionYearMethodResponse {
    fn from(y: TransactionYearMethod) -> Self {
        Self {
            year: y.year,
            payment_method: y.payment_method,
            total_transactions: y.total_transactions,
            total_amount: y.total_amount,
        }
    }
}

impl From<TransactionMonthAmount> for TransactionMonthAmountResponse {
    fn from(m: TransactionMonthAmount) -> Self {
        Self {
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<TransactionYearlyAmount> for TransactionYearlyAmountResponse {
    fn from(y: TransactionYearlyAmount) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
        }
    }
}

// response to proto
impl From<TransactionResponse> for TransactionResponseProto {
    fn from(r: TransactionResponse) -> Self {
        Self {
            id: r.id,
            transaction_no: r.transaction_no,
            card_number: r.card_number,
            amount: r.amount,
            payment_method: r.payment_method,
            merchant_id: r.merchant_id,
            transaction_time: r.transaction_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<TransactionResponseDeleteAt> for TransactionResponseDeleteAtProto {
    fn from(r: TransactionResponseDeleteAt) -> Self {
        Self {
            id: r.id,
            transaction_no: r.transaction_no,
            card_number: r.card_number,
            amount: r.amount,
            payment_method: r.payment_method,
            merchant_id: r.merchant_id,
            transaction_time: r.transaction_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<TransactionResponseMonthStatusSuccess> for TransactionResponseMonthStatusSuccessProto {
    fn from(r: TransactionResponseMonthStatusSuccess) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TransactionResponseYearStatusSuccess> for TransactionResponseYearStatusSuccessProto {
    fn from(r: TransactionResponseYearStatusSuccess) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TransactionResponseMonthStatusFailed> for TransactionResponseMonthStatusFailedProto {
    fn from(r: TransactionResponseMonthStatusFailed) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TransactionResponseYearStatusFailed> for TransactionResponseYearStatusFailedProto {
    fn from(r: TransactionResponseYearStatusFailed) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TransactionMonthMethodResponse> for TransactionMonthMethodResponseProto {
    fn from(r: TransactionMonthMethodResponse) -> Self {
        Self {
            month: r.month,
            payment_method: r.payment_method,
            total_transactions: r.total_transactions,
            total_amount: r.total_amount,
        }
    }
}

impl From<TransactionYearMethodResponse> for TransactionYearMethodResponseProto {
    fn from(r: TransactionYearMethodResponse) -> Self {
        Self {
            year: r.year,
            payment_method: r.payment_method,
            total_transactions: r.total_transactions,
            total_amount: r.total_amount,
        }
    }
}

impl From<TransactionMonthAmountResponse> for TransactionMonthAmountResponseProto {
    fn from(r: TransactionMonthAmountResponse) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount,
        }
    }
}

impl From<TransactionYearlyAmountResponse> for TransactionYearlyAmountResponseProto {
    fn from(r: TransactionYearlyAmountResponse) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
        }
    }
}

// proto to response
impl From<TransactionResponseProto> for TransactionResponse {
    fn from(p: TransactionResponseProto) -> Self {
        Self {
            id: p.id,
            transaction_no: p.transaction_no,
            card_number: p.card_number,
            amount: p.amount,
            payment_method: p.payment_method,
            merchant_id: p.merchant_id,
            transaction_time: p.transaction_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<TransactionResponseDeleteAtProto> for TransactionResponseDeleteAt {
    fn from(p: TransactionResponseDeleteAtProto) -> Self {
        Self {
            id: p.id,
            transaction_no: p.transaction_no,
            card_number: p.card_number,
            amount: p.amount,
            payment_method: p.payment_method,
            merchant_id: p.merchant_id,
            transaction_time: p.transaction_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<TransactionResponseMonthStatusSuccessProto> for TransactionResponseMonthStatusSuccess {
    fn from(p: TransactionResponseMonthStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TransactionResponseYearStatusSuccessProto> for TransactionResponseYearStatusSuccess {
    fn from(p: TransactionResponseYearStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TransactionResponseMonthStatusFailedProto> for TransactionResponseMonthStatusFailed {
    fn from(p: TransactionResponseMonthStatusFailedProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TransactionResponseYearStatusFailedProto> for TransactionResponseYearStatusFailed {
    fn from(p: TransactionResponseYearStatusFailedProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TransactionMonthMethodResponseProto> for TransactionMonthMethodResponse {
    fn from(p: TransactionMonthMethodResponseProto) -> Self {
        Self {
            month: p.month,
            payment_method: p.payment_method,
            total_transactions: p.total_transactions,
            total_amount: p.total_amount,
        }
    }
}

impl From<TransactionYearMethodResponseProto> for TransactionYearMethodResponse {
    fn from(p: TransactionYearMethodResponseProto) -> Self {
        Self {
            year: p.year,
            payment_method: p.payment_method,
            total_transactions: p.total_transactions,
            total_amount: p.total_amount,
        }
    }
}

impl From<TransactionMonthAmountResponseProto> for TransactionMonthAmountResponse {
    fn from(p: TransactionMonthAmountResponseProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<TransactionYearlyAmountResponseProto> for TransactionYearlyAmountResponse {
    fn from(p: TransactionYearlyAmountResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
