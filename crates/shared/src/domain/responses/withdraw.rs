use crate::{
    model::withdraw::{
        WithdrawModel, WithdrawModelMonthStatusFailed, WithdrawModelMonthStatusSuccess,
        WithdrawModelYearStatusFailed, WithdrawModelYearStatusSuccess, WithdrawMonthlyAmount,
        WithdrawYearlyAmount,
    },
    utils::parse_datetime,
};
use genproto::withdraw::{
    WithdrawMonthStatusFailedResponse as WithdrawResponseMonthStatusFailedProto,
    WithdrawMonthStatusSuccessResponse as WithdrawResponseMonthStatusSuccessProto,
    WithdrawMonthlyAmountResponse as WithdrawMonthlyAmountResponseProto,
    WithdrawResponse as WithdrawResponseProto,
    WithdrawResponseDeleteAt as WithdrawResponseDeleteAtProto,
    WithdrawYearStatusFailedResponse as WithdrawResponseYearStatusFailedProto,
    WithdrawYearStatusSuccessResponse as WithdrawResponseYearStatusSuccessProto,
    WithdrawYearlyAmountResponse as WithdrawYearlyAmountResponseProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponse {
    pub id: i32,
    #[serde(rename = "withdraw_no")]
    pub withdraw_no: String,
    #[serde(rename = "card_number")]
    pub card_number: String,
    #[serde(rename = "withdraw_amount")]
    pub withdraw_amount: i64,
    #[serde(rename = "withdraw_time")]
    pub withdraw_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponseDeleteAt {
    pub id: i32,
    #[serde(rename = "withdraw_no")]
    pub withdraw_no: String,
    #[serde(rename = "card_number")]
    pub card_number: String,
    #[serde(rename = "withdraw_amount")]
    pub withdraw_amount: i64,
    #[serde(rename = "withdraw_time")]
    pub withdraw_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
    #[serde(rename = "total_success")]
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponseYearStatusSuccess {
    pub year: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
    #[serde(rename = "total_success")]
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponseMonthStatusFailed {
    pub year: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
    pub month: String,
    #[serde(rename = "total_failed")]
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawResponseYearStatusFailed {
    pub year: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
    #[serde(rename = "total_failed")]
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawMonthlyAmountResponse {
    pub month: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WithdrawYearlyAmountResponse {
    pub year: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
}

// model to response
impl From<WithdrawModel> for WithdrawResponse {
    fn from(model: WithdrawModel) -> Self {
        Self {
            id: model.withdraw_id,
            withdraw_no: model.withdraw_no.to_string(),
            card_number: model.card_number,
            withdraw_amount: model.withdraw_amount as i64,
            withdraw_time: model.withdraw_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<WithdrawModel> for WithdrawResponseDeleteAt {
    fn from(model: WithdrawModel) -> Self {
        Self {
            id: model.withdraw_id,
            withdraw_no: model.withdraw_no.to_string(),
            card_number: model.card_number,
            withdraw_amount: model.withdraw_amount as i64,
            withdraw_time: model.withdraw_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<WithdrawModelMonthStatusSuccess> for WithdrawResponseMonthStatusSuccess {
    fn from(m: WithdrawModelMonthStatusSuccess) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_success: m.total_success,
        }
    }
}

impl From<WithdrawModelYearStatusSuccess> for WithdrawResponseYearStatusSuccess {
    fn from(y: WithdrawModelYearStatusSuccess) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_success: y.total_success,
        }
    }
}

impl From<WithdrawModelMonthStatusFailed> for WithdrawResponseMonthStatusFailed {
    fn from(m: WithdrawModelMonthStatusFailed) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_failed: m.total_failed,
        }
    }
}

impl From<WithdrawModelYearStatusFailed> for WithdrawResponseYearStatusFailed {
    fn from(y: WithdrawModelYearStatusFailed) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_failed: y.total_failed,
        }
    }
}

impl From<WithdrawMonthlyAmount> for WithdrawMonthlyAmountResponse {
    fn from(m: WithdrawMonthlyAmount) -> Self {
        Self {
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<WithdrawYearlyAmount> for WithdrawYearlyAmountResponse {
    fn from(y: WithdrawYearlyAmount) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
        }
    }
}

// response to proto
impl From<WithdrawResponse> for WithdrawResponseProto {
    fn from(r: WithdrawResponse) -> Self {
        Self {
            withdraw_id: r.id,
            withdraw_no: r.withdraw_no,
            card_number: r.card_number,
            withdraw_amount: r.withdraw_amount,
            withdraw_time: r.withdraw_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<WithdrawResponseDeleteAt> for WithdrawResponseDeleteAtProto {
    fn from(r: WithdrawResponseDeleteAt) -> Self {
        Self {
            withdraw_id: r.id,
            withdraw_no: r.withdraw_no,
            card_number: r.card_number,
            withdraw_amount: r.withdraw_amount,
            withdraw_time: r.withdraw_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<WithdrawResponseMonthStatusSuccess> for WithdrawResponseMonthStatusSuccessProto {
    fn from(r: WithdrawResponseMonthStatusSuccess) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<WithdrawResponseYearStatusSuccess> for WithdrawResponseYearStatusSuccessProto {
    fn from(r: WithdrawResponseYearStatusSuccess) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<WithdrawResponseMonthStatusFailed> for WithdrawResponseMonthStatusFailedProto {
    fn from(r: WithdrawResponseMonthStatusFailed) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<WithdrawResponseYearStatusFailed> for WithdrawResponseYearStatusFailedProto {
    fn from(r: WithdrawResponseYearStatusFailed) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<WithdrawMonthlyAmountResponse> for WithdrawMonthlyAmountResponseProto {
    fn from(r: WithdrawMonthlyAmountResponse) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount,
        }
    }
}

impl From<WithdrawYearlyAmountResponse> for WithdrawYearlyAmountResponseProto {
    fn from(r: WithdrawYearlyAmountResponse) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
        }
    }
}

// proto to response
impl From<WithdrawResponseProto> for WithdrawResponse {
    fn from(p: WithdrawResponseProto) -> Self {
        Self {
            id: p.withdraw_id,
            withdraw_no: p.withdraw_no,
            card_number: p.card_number,
            withdraw_amount: p.withdraw_amount,
            withdraw_time: p.withdraw_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<WithdrawResponseDeleteAtProto> for WithdrawResponseDeleteAt {
    fn from(p: WithdrawResponseDeleteAtProto) -> Self {
        Self {
            id: p.withdraw_id,
            withdraw_no: p.withdraw_no,
            card_number: p.card_number,
            withdraw_amount: p.withdraw_amount,
            withdraw_time: p.withdraw_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<WithdrawResponseMonthStatusSuccessProto> for WithdrawResponseMonthStatusSuccess {
    fn from(p: WithdrawResponseMonthStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<WithdrawResponseYearStatusSuccessProto> for WithdrawResponseYearStatusSuccess {
    fn from(p: WithdrawResponseYearStatusSuccessProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<WithdrawResponseMonthStatusFailedProto> for WithdrawResponseMonthStatusFailed {
    fn from(p: WithdrawResponseMonthStatusFailedProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<WithdrawResponseYearStatusFailedProto> for WithdrawResponseYearStatusFailed {
    fn from(p: WithdrawResponseYearStatusFailedProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<WithdrawMonthlyAmountResponseProto> for WithdrawMonthlyAmountResponse {
    fn from(p: WithdrawMonthlyAmountResponseProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<WithdrawYearlyAmountResponseProto> for WithdrawYearlyAmountResponse {
    fn from(p: WithdrawYearlyAmountResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
