use crate::{
    abstract_trait::transaction::{
        repository::statsbycard::amount::DynTransactionStatsAmountByCardRepository,
        service::statsbycard::amount::TransactionStatsAmountByCardServiceTrait,
    },
    domain::{
        requests::transaction::MonthYearPaymentMethod,
        responses::{ApiResponse, TransactionMonthAmountResponse, TransactionYearlyAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TransactionStatsAmountByCardService {
    amount: DynTransactionStatsAmountByCardRepository,
}

impl TransactionStatsAmountByCardService {
    pub async fn new(amount: DynTransactionStatsAmountByCardRepository) -> Self {
        Self { amount }
    }
}

#[async_trait]
impl TransactionStatsAmountByCardServiceTrait for TransactionStatsAmountByCardService {
    async fn get_monthly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly transaction amounts for card: ({}-{})",
            req.card_number, req.year,
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_monthly_amounts(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch monthly amounts for card  ({}-{}): {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionMonthAmountResponse> = amounts
            .into_iter()
            .map(TransactionMonthAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly transaction amount records for card ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transaction amounts for card in {}-{} retrieved successfully",
                req.card_number, req.year,
            ),
            data: response_data,
        })
    }

    async fn get_yearly_amounts(
        &self,
        req: &MonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly transaction amounts for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let amounts = self.amount.get_yearly_amounts(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly amounts for card {} in {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TransactionYearlyAmountResponse> = amounts
            .into_iter()
            .map(TransactionYearlyAmountResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly transaction amount records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transaction amounts for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
