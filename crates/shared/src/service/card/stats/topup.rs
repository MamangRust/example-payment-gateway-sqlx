use crate::{
    abstract_trait::card::{
        repository::stats::topup::DynCardStatsTopupRepository,
        service::stats::topup::CardStatsTopupServiceTrait,
    },
    domain::responses::{ApiResponse, CardResponseMonthAmount, CardResponseYearAmount},
    errors::ServiceError,
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::{error, info};

pub struct CardStatsTopupService {
    topup: DynCardStatsTopupRepository,
}

impl CardStatsTopupService {
    pub async fn new(topup: DynCardStatsTopupRepository) -> Self {
        Self { topup }
    }
}

#[async_trait]
impl CardStatsTopupServiceTrait for CardStatsTopupService {
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, ServiceError> {
        info!("📅 Fetching monthly top-up amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.topup.get_monthly_amount(year).await.map_err(|e| {
            error!("🗄️ Failed to retrieve monthly top-up data for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseMonthAmount> = amounts
            .into_iter()
            .map(CardResponseMonthAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} monthly top-up records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Monthly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }

    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, ServiceError> {
        info!("📆 Fetching yearly top-up amounts for year: {}", year);

        if !(2000..=2100).contains(&year) {
            let msg = "Year must be between 2000 and 2100".to_string();
            error!("Validation failed: {msg}");
            return Err(ServiceError::Custom(msg));
        }

        let amounts = self.topup.get_yearly_amount(year).await.map_err(|e| {
            error!("🗄️ Failed to retrieve yearly top-up data for year {year}: {e:?}");
            ServiceError::Repo(e)
        })?;

        let response_data: Vec<CardResponseYearAmount> = amounts
            .into_iter()
            .map(CardResponseYearAmount::from)
            .collect();

        info!(
            "✅ Successfully retrieved {} yearly top-up records for year {year}",
            response_data.len(),
        );

        Ok(ApiResponse {
            status: "success".to_string(),
            message: format!("Yearly top-up amounts for year {year} retrieved successfully"),
            data: response_data,
        })
    }
}
