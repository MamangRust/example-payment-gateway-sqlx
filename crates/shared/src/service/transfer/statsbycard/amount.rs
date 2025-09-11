use crate::{
    abstract_trait::transfer::{
        repository::statsbycard::amount::DynTransferStatsAmountByCardRepository,
        service::statsbycard::amount::TransferStatsAmountByCardServiceTrait,
    },
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransferStatsAmountByCardService {
    amount: DynTransferStatsAmountByCardRepository,
}

impl TransferStatsAmountByCardService {
    pub async fn new(amount: DynTransferStatsAmountByCardRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TransferStatsAmountByCardServiceTrait for TransferStatsAmountByCardService {
    async fn get_monthly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!(
            "💳➡️📊 Fetching monthly transfer amounts (as sender) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .amount
            .get_monthly_amounts_by_sender_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch monthly amounts (sender) for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transfer records (as sender) for card {} in {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_monthly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, ServiceError> {
        info!(
            "⬅️💳📊 Fetching monthly transfer amounts (as receiver) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .amount
            .get_monthly_amounts_by_receiver_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch monthly amounts (receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transfer records (as receiver) for card {} in {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amounts_by_sender(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!(
            "💳➡️📅 Fetching yearly transfer amounts (as sender) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .amount
            .get_yearly_amounts_by_sender_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly amounts (sender) for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transfer records (as sender) for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amounts_by_receiver(
        &self,
        req: &MonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, ServiceError> {
        info!(
            "⬅️💳📅 Fetching yearly transfer amounts (as receiver) for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self
            .amount
            .get_yearly_amounts_by_receiver_card(req)
            .await
            .map_err(|e| {
                error!(
                    "❌ Failed to fetch yearly amounts (receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transfer records (as receiver) for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
