use crate::{
    abstract_trait::topup::{
        repository::statsbycard::method::DynTopupStatsMethodByCardRepository,
        service::statsbycard::method::TopupStatsMethodByCardServiceTrait,
    },
    domain::{
        requests::topup::YearMonthMethod,
        responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TopupStatsMethodByCardService {
    method: DynTopupStatsMethodByCardRepository,
}

impl TopupStatsMethodByCardService {
    pub async fn new(method: DynTopupStatsMethodByCardRepository) -> Self {
        Self { method }
    }
}

#[async_trait]
impl TopupStatsMethodByCardServiceTrait for TopupStatsMethodByCardService {
    async fn get_monthly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError> {
        info!(
            "💳📊 Fetching monthly top-up methods for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_monthly_methods(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch monthly methods for card {} in year {}: {}",
                req.card_number, req.year, e
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupMonthMethodResponse> = methods
            .into_iter()
            .map(TopupMonthMethodResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} monthly method records for card {} in year {}",
            response_data.len(),
            req.card_number,
            req.year
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly top-up methods for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_yearly_methods(
        &self,
        req: &YearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError> {
        info!(
            "📅💳 Fetching yearly top-up methods for card: {} (Year: {})",
            req.card_number, req.year
        );

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let methods = self.method.get_yearly_methods(req).await.map_err(|e| {
            error!(
                "❌ Failed to fetch yearly methods for card {} in year {}: {e:?}",
                req.card_number, req.year,
            );
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupYearlyMethodResponse> = methods
            .into_iter()
            .map(TopupYearlyMethodResponse::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly method records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly top-up methods for card {} in year {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
