use crate::{
    abstract_trait::topup::{
        repository::stats::method::DynTopupStatsMethodRepository,
        service::stats::method::TopupStatsMethodServiceTrait,
    },
    domain::responses::{ApiResponse, TopupMonthMethodResponse, TopupYearlyMethodResponse},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct TopupStatsMethodService {
    method: DynTopupStatsMethodRepository,
}

impl TopupStatsMethodService {
    pub async fn new(method: DynTopupStatsMethodRepository) -> Self {
        Self { method }
    }
}

#[async_trait]
impl TopupStatsMethodServiceTrait for TopupStatsMethodService {
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, ServiceError> {
        info!("📅💳 Fetching monthly top-up methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_monthly_methods(year).await.map_err(|e| {
            error!("❌ Failed to retrieve monthly top-up methods for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupMonthMethodResponse> = methods
            .into_iter()
            .map(TopupMonthMethodResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} monthly top-up method records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly top-up methods for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, ServiceError> {
        info!("📆💳 Fetching yearly top-up methods for year: {year}");

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let methods = self.method.get_yearly_methods(year).await.map_err(|e| {
            error!("❌ Failed to retrieve yearly top-up methods for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<TopupYearlyMethodResponse> = methods
            .into_iter()
            .map(TopupYearlyMethodResponse::from)
            .collect();

        info!(
            "✅ Retrieved {} yearly top-up method records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly top-up methods for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
