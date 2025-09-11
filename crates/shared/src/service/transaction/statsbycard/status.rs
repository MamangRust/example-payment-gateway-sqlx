use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::status::DynTransactionStatsStatusByCardRepository,
        service::statsbycard::status::TransactionStatsStatusByCardServiceTrait,
    },
    domain::{
        requests::transaction::{
            MonthStatusTransactionCardNumber, YearStatusTransactionCardNumber,
        },
        responses::{
            ApiResponse, TransactionResponseMonthStatusFailed,
            TransactionResponseMonthStatusSuccess, TransactionResponseYearStatusFailed,
            TransactionResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionStatsStatusByCardService {
    status: DynTransactionStatsStatusByCardRepository,
}

impl TransactionStatsStatusByCardService {
    pub async fn new(status: DynTransactionStatsStatusByCardRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TransactionStatsStatusByCardServiceTrait for TransactionStatsStatusByCardService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful monthly transactions for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_success(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch successful monthly transactions for card {} ({}-{}): {e:?}",
                    req.card_number, req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful monthly transaction records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transactions for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful transactions for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_success(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly successful transactions for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransactionResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful transaction records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly transactions for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed monthly transactions for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_failed(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch failed monthly transactions for card {} ({}-{}): {e:?}",
                    req.card_number, req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransactionResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed monthly transaction records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transactions for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        req: &YearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed transactions for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_failed(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly failed transactions for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransactionResponseYearStatusFailed> = results
            .into_iter()
            .map(TransactionResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed transaction records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed yearly transactions for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
