use crate::{
    model::transfer::{
        TransferModel, TransferModelMonthStatusFailed, TransferModelMonthStatusSuccess,
        TransferModelYearStatusFailed, TransferModelYearStatusSuccess, TransferMonthAmount,
        TransferYearAmount,
    },
    utils::parse_datetime,
};
use genproto::transfer::{
    TransferMonthAmountResponse as TransferMonthAmountResponseProto,
    TransferMonthStatusFailedResponse as TransferResponseMonthStatusFailedProto,
    TransferMonthStatusSuccessResponse as TransferResponseMonthStatusSuccessProto,
    TransferResponse as TransferResponseProto,
    TransferResponseDeleteAt as TransferResponseDeleteAtProto,
    TransferYearAmountResponse as TransferYearAmountResponseProto,
    TransferYearStatusFailedResponse as TransferResponseYearStatusFailedProto,
    TransferYearStatusSuccessResponse as TransferResponseYearStatusSuccessProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponse {
    pub id: i32,
    pub transfer_no: String,
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: i64,
    pub transfer_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponseDeleteAt {
    pub id: i32,
    pub transfer_no: String,
    pub transfer_from: String,
    pub transfer_to: String,
    pub transfer_amount: i64,
    pub transfer_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponseMonthStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub month: String,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferMonthAmountResponse {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TransferYearAmountResponse {
    pub year: String,
    pub total_amount: i64,
}

// model to response
impl From<TransferModel> for TransferResponse {
    fn from(model: TransferModel) -> Self {
        Self {
            id: model.transfer_id,
            transfer_no: model.transfer_no.to_string(),
            transfer_from: model.transfer_from,
            transfer_to: model.transfer_to,
            transfer_amount: model.transfer_amount as i64,
            transfer_time: model.transfer_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TransferModel> for TransferResponseDeleteAt {
    fn from(model: TransferModel) -> Self {
        Self {
            id: model.transfer_id,
            transfer_no: model.transfer_no.to_string(),
            transfer_from: model.transfer_from,
            transfer_to: model.transfer_to,
            transfer_amount: model.transfer_amount as i64,
            transfer_time: model.transfer_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TransferModelMonthStatusSuccess> for TransferResponseMonthStatusSuccess {
    fn from(m: TransferModelMonthStatusSuccess) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_success: m.total_success,
        }
    }
}

impl From<TransferModelYearStatusSuccess> for TransferResponseYearStatusSuccess {
    fn from(y: TransferModelYearStatusSuccess) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_success: y.total_success,
        }
    }
}

impl From<TransferModelMonthStatusFailed> for TransferResponseMonthStatusFailed {
    fn from(m: TransferModelMonthStatusFailed) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_failed: m.total_failed,
        }
    }
}

impl From<TransferModelYearStatusFailed> for TransferResponseYearStatusFailed {
    fn from(y: TransferModelYearStatusFailed) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_failed: y.total_failed,
        }
    }
}

impl From<TransferMonthAmount> for TransferMonthAmountResponse {
    fn from(m: TransferMonthAmount) -> Self {
        Self {
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<TransferYearAmount> for TransferYearAmountResponse {
    fn from(y: TransferYearAmount) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
        }
    }
}

// respone to proto
impl From<TransferResponse> for TransferResponseProto {
    fn from(r: TransferResponse) -> Self {
        Self {
            id: r.id,
            transfer_no: r.transfer_no,
            transfer_from: r.transfer_from,
            transfer_to: r.transfer_to,
            transfer_amount: r.transfer_amount,
            transfer_time: r.transfer_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<TransferResponseDeleteAt> for TransferResponseDeleteAtProto {
    fn from(r: TransferResponseDeleteAt) -> Self {
        Self {
            id: r.id,
            transfer_no: r.transfer_no,
            transfer_from: r.transfer_from,
            transfer_to: r.transfer_to,
            transfer_amount: r.transfer_amount,
            transfer_time: r.transfer_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<TransferResponseMonthStatusSuccess> for TransferResponseMonthStatusSuccessProto {
    fn from(r: TransferResponseMonthStatusSuccess) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TransferResponseYearStatusSuccess> for TransferResponseYearStatusSuccessProto {
    fn from(r: TransferResponseYearStatusSuccess) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TransferResponseMonthStatusFailed> for TransferResponseMonthStatusFailedProto {
    fn from(r: TransferResponseMonthStatusFailed) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TransferResponseYearStatusFailed> for TransferResponseYearStatusFailedProto {
    fn from(r: TransferResponseYearStatusFailed) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TransferMonthAmountResponse> for TransferMonthAmountResponseProto {
    fn from(r: TransferMonthAmountResponse) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount,
        }
    }
}

impl From<TransferYearAmountResponse> for TransferYearAmountResponseProto {
    fn from(r: TransferYearAmountResponse) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
        }
    }
}

// proto to response
impl From<TransferResponseProto> for TransferResponse {
    fn from(p: TransferResponseProto) -> Self {
        Self {
            id: p.id,
            transfer_no: p.transfer_no,
            transfer_from: p.transfer_from,
            transfer_to: p.transfer_to,
            transfer_amount: p.transfer_amount,
            transfer_time: p.transfer_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<TransferResponseDeleteAtProto> for TransferResponseDeleteAt {
    fn from(p: TransferResponseDeleteAtProto) -> Self {
        Self {
            id: p.id,
            transfer_no: p.transfer_no,
            transfer_from: p.transfer_from,
            transfer_to: p.transfer_to,
            transfer_amount: p.transfer_amount,
            transfer_time: p.transfer_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<TransferResponseMonthStatusSuccessProto> for TransferResponseMonthStatusSuccess {
    fn from(p: TransferResponseMonthStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TransferResponseYearStatusSuccessProto> for TransferResponseYearStatusSuccess {
    fn from(p: TransferResponseYearStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TransferResponseMonthStatusFailedProto> for TransferResponseMonthStatusFailed {
    fn from(p: TransferResponseMonthStatusFailedProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TransferResponseYearStatusFailedProto> for TransferResponseYearStatusFailed {
    fn from(p: TransferResponseYearStatusFailedProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TransferMonthAmountResponseProto> for TransferMonthAmountResponse {
    fn from(p: TransferMonthAmountResponseProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<TransferYearAmountResponseProto> for TransferYearAmountResponse {
    fn from(p: TransferYearAmountResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
