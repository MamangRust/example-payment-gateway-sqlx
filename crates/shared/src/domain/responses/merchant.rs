use crate::{
    model::merchant::{
        MerchantModel, MerchantMonthlyAmount, MerchantMonthlyPaymentMethod,
        MerchantMonthlyTotalAmount, MerchantTransactionsModel, MerchantYearlyAmount,
        MerchantYearlyPaymentMethod, MerchantYearlyTotalAmount,
    },
    utils::parse_datetime,
};
use genproto::merchant::{
    MerchantResponse as MerchantResponseProto,
    MerchantResponseDeleteAt as MerchantResponseDeleteAtProto,
    MerchantResponseMonthlyAmount as MerchantResponseMonthlyAmountProto,
    MerchantResponseMonthlyPaymentMethod as MerchantResponseMonthlyPaymentMethodProto,
    MerchantResponseMonthlyTotalAmount as MerchantMonthlyTotalAmountProto,
    MerchantResponseYearlyAmount as MerchantResponseYearlyAmountProto,
    MerchantResponseYearlyPaymentMethod as MerchantResponseYearlyPaymentMethodProto,
    MerchantResponseYearlyTotalAmount as MerchantResponseYearlyTotalAmountProto,
    MerchantTransactionResponse as MerchantTransactionResponseProto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponse {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub api_key: String,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseDeleteAt {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub api_key: String,
    pub status: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantTransactionResponse {
    pub id: i32,
    pub card_number: String,
    pub amount: i32,
    pub payment_method: String,
    pub merchant_id: i32,
    pub merchant_name: String,
    pub transaction_time: String,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseMonthlyPaymentMethod {
    pub month: String,
    pub payment_method: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseYearlyPaymentMethod {
    pub year: String,
    pub payment_method: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseMonthlyAmount {
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseYearlyAmount {
    pub year: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseMonthlyTotalAmount {
    pub year: String,
    pub month: String,
    pub total_amount: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MerchantResponseYearlyTotalAmount {
    pub year: String,
    pub total_amount: i64,
}

// model to response
impl From<MerchantModel> for MerchantResponse {
    fn from(model: MerchantModel) -> Self {
        Self {
            id: model.merchant_id,
            name: model.name,
            user_id: model.user_id,
            api_key: model.api_key,
            status: model.status,
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<MerchantModel> for MerchantResponseDeleteAt {
    fn from(model: MerchantModel) -> Self {
        Self {
            id: model.merchant_id,
            name: model.name,
            user_id: model.user_id,
            api_key: model.api_key,
            status: model.status,
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<MerchantTransactionsModel> for MerchantTransactionResponse {
    fn from(model: MerchantTransactionsModel) -> Self {
        Self {
            id: model.transaction_id,
            card_number: model.card_number,
            amount: model.amount,
            payment_method: model.payment_method,
            merchant_id: model.merchant_id,
            merchant_name: model.merchant_name,
            transaction_time: model.transaction_time.to_string(),
            created_at: model.created_at.map(|dt| dt.to_string()),
            updated_at: model.updated_at.map(|dt| dt.to_string()),
            deleted_at: model.deleted_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<MerchantMonthlyPaymentMethod> for MerchantResponseMonthlyPaymentMethod {
    fn from(m: MerchantMonthlyPaymentMethod) -> Self {
        Self {
            month: m.month,
            payment_method: m.payment_method,
            total_amount: m.total_amount,
        }
    }
}

impl From<MerchantYearlyPaymentMethod> for MerchantResponseYearlyPaymentMethod {
    fn from(m: MerchantYearlyPaymentMethod) -> Self {
        Self {
            year: m.year,
            payment_method: m.payment_method,
            total_amount: m.total_amount,
        }
    }
}

impl From<MerchantMonthlyAmount> for MerchantResponseMonthlyAmount {
    fn from(m: MerchantMonthlyAmount) -> Self {
        Self {
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<MerchantYearlyAmount> for MerchantResponseYearlyAmount {
    fn from(m: MerchantYearlyAmount) -> Self {
        Self {
            year: m.year,
            total_amount: m.total_amount,
        }
    }
}

impl From<MerchantMonthlyTotalAmount> for MerchantResponseMonthlyTotalAmount {
    fn from(m: MerchantMonthlyTotalAmount) -> Self {
        Self {
            year: m.year,
            month: m.month,
            total_amount: m.total_amount,
        }
    }
}

impl From<MerchantYearlyTotalAmount> for MerchantResponseYearlyTotalAmount {
    fn from(m: MerchantYearlyTotalAmount) -> Self {
        Self {
            year: m.year,
            total_amount: m.total_amount,
        }
    }
}

// response to proto
impl From<MerchantResponse> for MerchantResponseProto {
    fn from(r: MerchantResponse) -> Self {
        Self {
            id: r.id,
            name: r.name,
            user_id: r.user_id,
            api_key: r.api_key,
            status: r.status,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
        }
    }
}

impl From<MerchantResponseDeleteAt> for MerchantResponseDeleteAtProto {
    fn from(r: MerchantResponseDeleteAt) -> Self {
        Self {
            id: r.id,
            name: r.name,
            user_id: r.user_id,
            api_key: r.api_key,
            status: r.status,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<MerchantTransactionResponse> for MerchantTransactionResponseProto {
    fn from(r: MerchantTransactionResponse) -> Self {
        Self {
            id: r.id,
            card_number: r.card_number,
            amount: r.amount,
            payment_method: r.payment_method,
            merchant_id: r.merchant_id,
            merchant_name: r.merchant_name,
            transaction_time: r.transaction_time,
            created_at: r.created_at.unwrap_or_default(),
            updated_at: r.updated_at.unwrap_or_default(),
            deleted_at: Some(r.deleted_at.unwrap_or_default()),
        }
    }
}

impl From<MerchantResponseMonthlyPaymentMethod> for MerchantResponseMonthlyPaymentMethodProto {
    fn from(r: MerchantResponseMonthlyPaymentMethod) -> Self {
        Self {
            month: r.month,
            payment_method: r.payment_method,
            total_amount: r.total_amount as i64,
        }
    }
}

impl From<MerchantResponseYearlyPaymentMethod> for MerchantResponseYearlyPaymentMethodProto {
    fn from(r: MerchantResponseYearlyPaymentMethod) -> Self {
        Self {
            year: r.year,
            payment_method: r.payment_method,
            total_amount: r.total_amount as i64,
        }
    }
}

impl From<MerchantResponseMonthlyAmount> for MerchantResponseMonthlyAmountProto {
    fn from(r: MerchantResponseMonthlyAmount) -> Self {
        Self {
            month: r.month,
            total_amount: r.total_amount as i64,
        }
    }
}

impl From<MerchantResponseYearlyAmount> for MerchantResponseYearlyAmountProto {
    fn from(r: MerchantResponseYearlyAmount) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount as i64,
        }
    }
}

impl From<MerchantResponseMonthlyTotalAmount> for MerchantMonthlyTotalAmountProto {
    fn from(r: MerchantResponseMonthlyTotalAmount) -> Self {
        Self {
            year: r.year,
            month: r.month,
            total_amount: r.total_amount as i64,
        }
    }
}

impl From<MerchantResponseYearlyTotalAmount> for MerchantResponseYearlyTotalAmountProto {
    fn from(r: MerchantResponseYearlyTotalAmount) -> Self {
        Self {
            year: r.year,
            total_amount: r.total_amount as i64,
        }
    }
}

// proto to response
impl From<MerchantResponseProto> for MerchantResponse {
    fn from(p: MerchantResponseProto) -> Self {
        Self {
            id: p.id,
            name: p.name,
            user_id: p.user_id,
            api_key: p.api_key,
            status: p.status,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
        }
    }
}

impl From<MerchantResponseDeleteAtProto> for MerchantResponseDeleteAt {
    fn from(p: MerchantResponseDeleteAtProto) -> Self {
        Self {
            id: p.id,
            name: p.name,
            user_id: p.user_id,
            api_key: p.api_key,
            status: p.status,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<MerchantTransactionResponseProto> for MerchantTransactionResponse {
    fn from(p: MerchantTransactionResponseProto) -> Self {
        Self {
            id: p.id,
            card_number: p.card_number,
            amount: p.amount,
            payment_method: p.payment_method,
            merchant_id: p.merchant_id,
            merchant_name: p.merchant_name,
            transaction_time: p.transaction_time,
            created_at: parse_datetime(&p.created_at),
            updated_at: parse_datetime(&p.updated_at),
            deleted_at: p.deleted_at.as_deref().and_then(parse_datetime),
        }
    }
}

impl From<MerchantResponseMonthlyPaymentMethodProto> for MerchantResponseMonthlyPaymentMethod {
    fn from(p: MerchantResponseMonthlyPaymentMethodProto) -> Self {
        Self {
            month: p.month,
            payment_method: p.payment_method,
            total_amount: p.total_amount,
        }
    }
}

impl From<MerchantResponseYearlyPaymentMethodProto> for MerchantResponseYearlyPaymentMethod {
    fn from(p: MerchantResponseYearlyPaymentMethodProto) -> Self {
        Self {
            year: p.year,
            payment_method: p.payment_method,
            total_amount: p.total_amount,
        }
    }
}

impl From<MerchantResponseMonthlyAmountProto> for MerchantResponseMonthlyAmount {
    fn from(p: MerchantResponseMonthlyAmountProto) -> Self {
        Self {
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<MerchantResponseYearlyAmountProto> for MerchantResponseYearlyAmount {
    fn from(p: MerchantResponseYearlyAmountProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}

impl From<MerchantMonthlyTotalAmountProto> for MerchantResponseMonthlyTotalAmount {
    fn from(p: MerchantMonthlyTotalAmountProto) -> Self {
        Self {
            year: p.year,
            month: p.month,
            total_amount: p.total_amount,
        }
    }
}

impl From<MerchantResponseYearlyTotalAmountProto> for MerchantResponseYearlyTotalAmount {
    fn from(p: MerchantResponseYearlyTotalAmountProto) -> Self {
        Self {
            year: p.year,
            total_amount: p.total_amount,
        }
    }
}
