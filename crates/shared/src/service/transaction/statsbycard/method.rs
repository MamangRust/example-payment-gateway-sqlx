use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::method::DynTransactionStatsMethodByCardRepository,
        service::statsbycard::method::TransactionStatsMethodByCardServiceTrait,
    },
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthMethodResponse, TransactionYearMethodResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionStatsMethodByCardService {
    method: DynTransactionStatsMethodByCardRepository,
}

impl TransactionStatsMethodByCardService {
    pub async fn new(method: DynTransactionStatsMethodByCardRepository) -> Self {
        Self { method }
    }
}

#[async_trait]
impl TransactionStatsMethodByCardServiceTrait for TransactionStatsMethodByCardService {
    async fn get_monthly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly transaction methods for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_monthly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch monthly transaction methods for card {} in {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionMonthMethodResponse> = methods
            .into_iter()
            .map(TransactionMonthMethodResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transaction method records for card {} in {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction methods for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_method(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly transaction methods for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_yearly_method(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly transaction methods for card {} in {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionYearMethodResponse> = methods
            .into_iter()
            .map(TransactionYearMethodResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transaction method records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction methods for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
