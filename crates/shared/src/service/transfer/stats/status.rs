use crate::{
    abstract_trait::transfer::{
        repository::stats::status::DynTransferStatsStatusRepository,
        service::stats::status::TransferStatsStatusServiceTrait,
    },
    domain::{
        requests::transfer::MonthStatusTransfer,
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

pub struct TransferStatsStatusService {
    status: DynTransferStatsStatusRepository,
}

impl TransferStatsStatusService {
    pub async fn new(status: DynTransferStatsStatusRepository) -> Self {
        Self { status }
    }
}

#[async_trait]
impl TransferStatsStatusServiceTrait for TransferStatsStatusService {
    async fn get_month_status_success(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, ServiceError> {
        info!(
            "📊✅ Fetching successful transfers for month: {}-{}",
            req.year, req.month
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
                    "❌ Failed to fetch successful transfers for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseMonthStatusSuccess> = results
            .into_iter()
            .map(TransferResponseMonthStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} successful transfer records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved successful transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, ServiceError> {
        info!("📅✅ Fetching yearly successful transfers for year: {year}",);

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
                error!("❌ Failed to fetch yearly successful transfers for {year}: {e:?}");
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseYearStatusSuccess> = results
            .into_iter()
            .map(TransferResponseYearStatusSuccess::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly successful transfer records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved successful yearly transfers for year {year}"),
            data: response_data,
        })
    }

    async fn get_month_status_failed(
        &self,
        req: &MonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, ServiceError> {
        info!(
            "📊❌ Fetching failed transfers for month: {}-{}",
            req.year, req.month
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
                    "❌ Failed to fetch failed transfers for {}-{}: {e:?}",
                    req.year, req.month,
                );
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseMonthStatusFailed> = results
            .into_iter()
            .map(TransferResponseMonthStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} failed transfer records for {}-{}",
            response_data.len(),
            req.year,
            req.month
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Successfully retrieved failed transfers for {}-{}",
                req.year, req.month
            ),
            data: response_data,
        })
    }

    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, ServiceError> {
        info!("📅❌ Fetching yearly failed transfers for year: {}", year);

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
                error!("❌ Failed to fetch yearly failed transfers for {year}: {e:?}",);
                ServiceError::Repo(e)
            })?;

        let response_data: Vec<TransferResponseYearStatusFailed> = results
            .into_iter()
            .map(TransferResponseYearStatusFailed::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly failed transfer records for {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Successfully retrieved failed yearly transfers for year {year}",),
            data: response_data,
        })
    }
}
