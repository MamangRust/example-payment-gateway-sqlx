use crate::{
    abstract_trait::topup::{
        repository::stats::status::DynTopupStatsStatusRepository,
        service::stats::status::TopupStatsStatusServiceTrait,
    },
    domain::{
        requests::topup::MonthTopupStatus,
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

pub struct TopupStatsStatusService {
    status: DynTopupStatsStatusRepository,
}

impl TopupStatsStatusService {
    pub async fn new(status: DynTopupStatsStatusRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TopupStatsStatusServiceTrait for TopupStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊 Fetching successful top-ups for month: {} and year: {}",
            req.month, req.year
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
                    "❌ Failed to fetch successful top-ups for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TopupResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Successfully fetched {} successful top-up records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful top-ups for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, ServiceError> {
        info!("📊 Fetching yearly successful top-ups for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let results = self
            .status
            .get_yearly_status_success(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch yearly successful top-ups for {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseYearStatusSuccess> = results
            .into_iter()
            .map(TopupResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly successful top-up records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful top-ups for year {year}"),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊 Fetching failed top-ups for month: {} and year: {}",
            req.month, req.year
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
                    "❌ Failed to fetch failed top-ups for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseMonthStatusFailed> = results
            .into_iter()
            .map(TopupResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Successfully fetched {} failed top-up records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed top-ups for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, ServiceError> {
        info!("📊 Fetching yearly failed top-ups for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let results = self
            .status
            .get_yearly_status_failed(year)
            .await
            .map_err(|e| {
                error!("❌ Failed to fetch yearly failed top-ups for {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TopupResponseYearStatusFailed> = results
            .into_iter()
            .map(TopupResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Successfully fetched {} yearly failed top-up records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed top-ups for year {year}"),
            data: response_data,
        })
    }
}
