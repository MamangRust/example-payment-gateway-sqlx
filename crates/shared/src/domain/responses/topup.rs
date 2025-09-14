use crate::{
    model::topup::{
        TopupModel, TopupModelMonthStatusFailed, TopupModelMonthStatusSuccess,
        TopupModelYearStatusFailed, TopupModelYearStatusSuccess, TopupMonthAmount,
        TopupMonthMethod, TopupYearlyAmount, TopupYearlyMethod,
    },
    utils::parse_datetime,
};
use genproto::topup::{
    TopupMonthAmountResponse as TopupMonthAmountResponseProto,
    TopupMonthMethodResponse as TopupMonthMethodResponseProto,
    TopupMonthStatusFailedResponse as TopupMonthStatusFailedResponseProto,
    TopupMonthStatusSuccessResponse as TopupMonthStatusSuccessResponseProto,
    TopupResponse as TopupResponseProto, TopupResponseDeleteAt as TopupResponseDeleteAtProto,
    TopupYearStatusFailedResponse as TopupYearStatusFailedResponseProto,
    TopupYearStatusSuccessResponse as TopupYearStatusSuccessResponseProto,
    TopupYearlyAmountResponse as TopupYearlyAmountResponseProto,
    TopupYearlyMethodResponse as TopupYearlyMethodResponseProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponse {
    pub id: i32,
    pub card_number: String,
    pub topup_no: String,
    pub topup_amount: i64,
    pub topup_method: String,
    pub topup_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponseDeleteAt {
    pub id: i32,
    pub card_number: String,
    pub topup_no: String,
    pub topup_amount: i64,
    pub topup_method: String,
    pub topup_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponseMonthStatusSuccess {
    pub year: String,
    pub month: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponseYearStatusSuccess {
    pub year: String,
    pub total_amount: i64,
    pub total_success: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponseMonthStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub month: String,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupResponseYearStatusFailed {
    pub year: String,
    pub total_amount: i64,
    pub total_failed: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupMonthMethodResponse {
    pub month: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupYearlyMethodResponse {
    pub year: String,
    pub topup_method: String,
    pub total_topups: i32,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupMonthAmountResponse {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TopupYearlyAmountResponse {
    pub year: String,
    pub total_amount: i64,
}

// model to response
impl From<TopupModel> for TopupResponse {
    fn from(model: TopupModel) -> Self {
        Self {
            id: model.topup_id,
            card_number: model.card_number,
            topup_no: model.topup_no.to_string(),
            topup_amount: model.topup_amount,
            topup_method: model.topup_method,
            topup_time: model.topup_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TopupModel> for TopupResponseDeleteAt {
    fn from(model: TopupModel) -> Self {
        Self {
            id: model.topup_id,
            card_number: model.card_number,
            topup_no: model.topup_no.to_string(),
            topup_amount: model.topup_amount,
            topup_method: model.topup_method,
            topup_time: model.topup_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TopupModelMonthStatusSuccess> for TopupResponseMonthStatusSuccess {
    fn from(m: TopupModelMonthStatusSuccess) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_success: m.total_success,
        }
    }
}

impl From<TopupModelYearStatusSuccess> for TopupResponseYearStatusSuccess {
    fn from(y: TopupModelYearStatusSuccess) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_success: y.total_success,
        }
    }
}

impl From<TopupModelMonthStatusFailed> for TopupResponseMonthStatusFailed {
    fn from(m: TopupModelMonthStatusFailed) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
            total_failed: m.total_failed,
        }
    }
}

impl From<TopupModelYearStatusFailed> for TopupResponseYearStatusFailed {
    fn from(y: TopupModelYearStatusFailed) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
            total_failed: y.total_failed,
        }
    }
}

impl From<TopupMonthMethod> for TopupMonthMethodResponse {
    fn from(m: TopupMonthMethod) -> Self {
        Self {
            month: m.month,
            topup_method: m.topup_method,
            total_topups: m.total_topups,
            total_amount: m.total_amount,
        }
    }
}

impl From<TopupYearlyMethod> for TopupYearlyMethodResponse {
    fn from(y: TopupYearlyMethod) -> Self {
        Self {
            year: y.year,
            topup_method: y.topup_method,
            total_topups: y.total_topups,
            total_amount: y.total_amount,
        }
    }
}

impl From<TopupMonthAmount> for TopupMonthAmountResponse {
    fn from(m: TopupMonthAmount) -> Self {
        Self {
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<TopupYearlyAmount> for TopupYearlyAmountResponse {
    fn from(y: TopupYearlyAmount) -> Self {
        Self {
            year: y.year,
            total_amount: y.total_amount,
        }
    }
}

// response to proto
impl From<TopupResponse> for TopupResponseProto {
    fn from(r: TopupResponse) -> Self {
        Self {
            id: r.id,
            card_number: r.card_number,
            topup_no: r.topup_no,
            topup_amount: r.topup_amount,
            topup_method: r.topup_method,
            topup_time: r.topup_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<TopupResponseDeleteAt> for TopupResponseDeleteAtProto {
    fn from(r: TopupResponseDeleteAt) -> Self {
        Self {
            id: r.id,
            card_number: r.card_number,
            topup_no: r.topup_no,
            topup_amount: r.topup_amount,
            topup_method: r.topup_method,
            topup_time: r.topup_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<TopupResponseMonthStatusSuccess> for TopupMonthStatusSuccessResponseProto {
    fn from(r: TopupResponseMonthStatusSuccess) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TopupResponseYearStatusSuccess> for TopupYearStatusSuccessResponseProto {
    fn from(r: TopupResponseYearStatusSuccess) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_success: r.total_success,
        }
    }
}

impl From<TopupResponseMonthStatusFailed> for TopupMonthStatusFailedResponseProto {
    fn from(r: TopupResponseMonthStatusFailed) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TopupResponseYearStatusFailed> for TopupYearStatusFailedResponseProto {
    fn from(r: TopupResponseYearStatusFailed) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
            total_failed: r.total_failed,
        }
    }
}

impl From<TopupMonthMethodResponse> for TopupMonthMethodResponseProto {
    fn from(r: TopupMonthMethodResponse) -> Self {
        Self {
            month: r.month,
            topup_method: r.topup_method,
            total_topups: r.total_topups,
            total_amount: r.total_amount,
        }
    }
}

impl From<TopupYearlyMethodResponse> for TopupYearlyMethodResponseProto {
    fn from(r: TopupYearlyMethodResponse) -> Self {
        Self {
            year: r.year,
            topup_method: r.topup_method,
            total_topups: r.total_topups,
            total_amount: r.total_amount,
        }
    }
}

impl From<TopupMonthAmountResponse> for TopupMonthAmountResponseProto {
    fn from(r: TopupMonthAmountResponse) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount,
        }
    }
}

impl From<TopupYearlyAmountResponse> for TopupYearlyAmountResponseProto {
    fn from(r: TopupYearlyAmountResponse) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount,
        }
    }
}

// proto to response
impl From<TopupResponseProto> for TopupResponse {
    fn from(p: TopupResponseProto) -> Self {
        Self {
            id: p.id,
            card_number: p.card_number,
            topup_no: p.topup_no,
            topup_amount: p.topup_amount,
            topup_method: p.topup_method,
            topup_time: p.topup_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<TopupResponseDeleteAtProto> for TopupResponseDeleteAt {
    fn from(p: TopupResponseDeleteAtProto) -> Self {
        Self {
            id: p.id,
            card_number: p.card_number,
            topup_no: p.topup_no,
            topup_amount: p.topup_amount,
            topup_method: p.topup_method,
            topup_time: p.topup_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<TopupMonthStatusSuccessResponseProto> for TopupResponseMonthStatusSuccess {
    fn from(p: TopupMonthStatusSuccessResponseProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TopupYearStatusSuccessResponseProto> for TopupResponseYearStatusSuccess {
    fn from(p: TopupYearStatusSuccessResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_success: p.total_success,
        }
    }
}

impl From<TopupMonthStatusFailedResponseProto> for TopupResponseMonthStatusFailed {
    fn from(p: TopupMonthStatusFailedResponseProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TopupYearStatusFailedResponseProto> for TopupResponseYearStatusFailed {
    fn from(p: TopupYearStatusFailedResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
            total_failed: p.total_failed,
        }
    }
}

impl From<TopupMonthMethodResponseProto> for TopupMonthMethodResponse {
    fn from(p: TopupMonthMethodResponseProto) -> Self {
        Self {
            month: p.month,
            topup_method: p.topup_method,
            total_topups: p.total_topups,
            total_amount: p.total_amount,
        }
    }
}

impl From<TopupYearlyMethodResponseProto> for TopupYearlyMethodResponse {
    fn from(p: TopupYearlyMethodResponseProto) -> Self {
        Self {
            year: p.year,
            topup_method: p.topup_method,
            total_topups: p.total_topups,
            total_amount: p.total_amount,
        }
    }
}

impl From<TopupMonthAmountResponseProto> for TopupMonthAmountResponse {
    fn from(p: TopupMonthAmountResponseProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<TopupYearlyAmountResponseProto> for TopupYearlyAmountResponse {
    fn from(p: TopupYearlyAmountResponseProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
