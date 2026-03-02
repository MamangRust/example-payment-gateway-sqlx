use crate::{
    abstract_trait::transfer::{
        repository::statsbycard::amount::DynTransferStatsAmountByCardRepository,
        service::statsbycard::amount::TransferStatsAmountByCardServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transfer::MonthYearCardNumber,
        responses::{ApiResponse, TransferMonthAmountResponse, TransferYearAmountResponse},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct TransferStatsAmountByCardService {
    pub amount: DynTransferStatsAmountByCardRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransferStatsAmountByCardService {
    pub fn new(
        amount: DynTransferStatsAmountByCardRepository,
        shared: &SharedResources,
    ) -> Result<Self> {
        Ok(Self {
            amount,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
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

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amounts_by_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_amounts_by_sender"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:monthly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (as sender) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts_by_sender_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer records (as sender) for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transfer amounts (as sender) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer amounts (as sender) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transfer amounts (as sender): {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transfer records (as sender) for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
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

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_monthly_amounts_by_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "monthly_amounts_by_receiver"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:monthly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferMonthAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transfer amounts (as receiver) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_monthly_amounts_by_receiver_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} monthly transfer records (as receiver) for card {} in {}",
                    amounts.len(),
                    req.card_number,
                    req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Monthly transfer amounts (as receiver) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve monthly transfer amounts (as receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve monthly transfer amounts (as receiver): {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferMonthAmountResponse> = amounts
            .into_iter()
            .map(TransferMonthAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Monthly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} monthly transfer records (as receiver) for card {} in {}",
            response.data.len(),
            req.card_number,
            req.year
        );

        Ok(response)
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

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amounts_by_sender",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_amounts_by_sender"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_amounts_by_sender:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as sender) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (as sender) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts_by_sender_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer records (as sender) for card {}",
                    amounts.len(),
                    req.card_number
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transfer amounts (as sender) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer amounts (as sender) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly transfer amounts (as sender): {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as sender) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transfer records (as sender) for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
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

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "get_yearly_amounts_by_receiver",
            vec![
                KeyValue::new("component", "transfer"),
                KeyValue::new("operation", "yearly_amounts_by_receiver"),
                KeyValue::new("card_number", req.card_number.clone()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut request = Request::new(req);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transfer:yearly_amounts_by_receiver:card:{}:year:{}",
            req.card_number, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransferYearAmountResponse>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transfer amounts (as receiver) in cache for card: {} (Year: {})",
                req.card_number, req.year
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transfer amounts (as receiver) retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let amounts = match self.amount.get_yearly_amounts_by_receiver_card(req).await {
            Ok(amounts) => {
                info!(
                    "✅ Successfully retrieved {} yearly transfer records (as receiver) for card {}",
                    amounts.len(),
                    req.card_number
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Yearly transfer amounts (as receiver) retrieved successfully",
                    )
                    .await;
                amounts
            }
            Err(e) => {
                error!(
                    "❌ Failed to retrieve yearly transfer amounts (as receiver) for card {} in {}: {e:?}",
                    req.card_number, req.year
                );
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!(
                            "Failed to retrieve yearly transfer amounts (as receiver): {:?}",
                            e
                        ),
                    )
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let response_data: Vec<TransferYearAmountResponse> = amounts
            .into_iter()
            .map(TransferYearAmountResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: format!(
                "Yearly transfer amounts (as receiver) for card {} in {} retrieved successfully",
                req.card_number, req.year
            ),
            data: response_data,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} yearly transfer records (as receiver) for card {}",
            response.data.len(),
            req.card_number
        );

        Ok(response)
    }
}
