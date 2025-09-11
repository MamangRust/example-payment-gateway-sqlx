use crate::{
    abstract_trait::topup::{
        repository::statsbycard::status::DynTopupStatsStatusByCardRepository,
        service::statsbycard::status::TopupStatsStatusByCardServiceTrait,
    },
    domain::{
        requests::topup::{MonthTopupStatusCardNumber, YearTopupStatusCardNumber},
        responses::{
            ApiResponse, TopupResponseMonthStatusFailed, TopupResponseMonthStatusSuccess,
            TopupResponseYearStatusFailed, TopupResponseYearStatusSuccess,
        },
    },
    errors::{ServiceError, format_validation_errors},
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};
use validator::Validate;

pub struct TopupStatsStatusByCardService {
    status: DynTopupStatsStatusByCardRepository,
}

impl TopupStatsStatusByCardService {
    pub async fn new(status: DynTopupStatsStatusByCardRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TopupStatsStatusByCardServiceTrait for TopupStatsStatusByCardService {
    async fn get_month_status_success(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful top-ups for card: {} ({}-{})",
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
                    "❌ Failed to fetch successful monthly top-ups for card {} ({}-{}): {e:?}",
                    req.card_number, req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TopupResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful top-up records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful top-ups for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError> {
        info!(
            "📅✅ Fetching yearly successful top-ups for card: {} (Year: {})",
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
                    "❌ Failed to fetch yearly successful top-ups for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseYearStatusSuccess> = results
            .into_iter()
            .map(TopupResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful top-up records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful yearly top-ups for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed top-ups for card: {} ({}-{})",
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
                    "❌ Failed to fetch failed monthly top-ups for card {} ({}-{}): {e:?}",
                    req.card_number, req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseMonthStatusFailed> = results
            .into_iter()
            .map(TopupResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed top-up records for card {} ({}-{})",
            response_data.len(),
            req.card_number,
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed top-ups for card {} in {}-{}",
                req.card_number, req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        req: &YearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError> {
        info!(
            "📅❌ Fetching yearly failed top-ups for card: {} (Year: {})",
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
                    "❌ Failed to fetch yearly failed top-ups for card {} in {}: {e:?}",
                    req.card_number, req.year,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseYearStatusFailed> = results
            .into_iter()
            .map(TopupResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed top-up records for card {}",
            response_data.len(),
            req.card_number
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed yearly top-ups for card {} in {}",
                req.card_number, req.year
            ),
            data: response_data,
        })
    }
}
