use crate::{
    abstract_trait::transfer::{
        repository::statsbycard::status::DynTransferStatsStatusByCardRepository,
        service::statsbycard::status::TransferStatsStatusByCardServiceTrait,
    },
    domain::{
        requests::transfer::{MonthStatusTransferCardNumber, YearStatusTransferCardNumber},
        responses::{
            ApiResponse, TransferResponseMonthStatusFailed, TransferResponseMonthStatusSuccess,
            TransferResponseYearStatusFailed, TransferResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransferStatsStatusByCardService {
    status: DynTransferStatsStatusByCardRepository,
}

impl TransferStatsStatusByCardService {
    pub async fn new(status: DynTransferStatsStatusByCardRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TransferStatsStatusByCardServiceTrait for TransferStatsStatusByCardService {
    async fn get_month_status_success_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful monthly transfers for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_success(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch successful monthly transfers for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransferResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful monthly transfer records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transfers for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful transfers for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_success(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly successful transfers for card {} in {}: {}",
                    req.card_number, req.year, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransferResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly transfers for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_month_status_failed_by_card(
        &self,
        req: &MonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed monthly transfers for card: {} ({}-{})",
            req.card_number, req.year, req.month
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_month_status_failed(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch failed monthly transfers for card {} ({}-{}): {}",
                    req.card_number, req.year, req.month, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransferResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed monthly transfer records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transfers for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed_by_card(
        &self,
        req: &YearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed transfers for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {}", error_msg);
            return Err(ServiceError::Custom(error_msg));
        }

        let results = self
            .status
            .get_yearly_status_failed(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly failed transfers for card {} in {}: {}",
                    req.card_number, req.year, e
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseYearStatusFailed> = results
            .into_iter()
            .map(TransferResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed transfer records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed yearly transfers for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
