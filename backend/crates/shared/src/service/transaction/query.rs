use crate::{
    abstract_trait::transaction::{
        repository::query::DynTransactionQueryRepository,
        service::query::TransactionQueryServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::transaction::{FindAllTransactionCardNumber, FindAllTransactions},
        responses::{
            ApiResponse, ApiResponsePagination, Pagination, TransactionResponse,
            TransactionResponseDeleteAt,
        },
    },
    errors::ServiceError,
    observability::{Method, TracingMetrics},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};

pub struct TransactionQueryService {
    pub query: DynTransactionQueryRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl TransactionQueryService {
    pub fn new(query: DynTransactionQueryRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            query,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl TransactionQueryServiceTrait for TransactionQueryService {
    async fn find_all(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🔍 Searching all transactions | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_all_transactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:find_all:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} transactions in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self.query.find_all(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} transactions", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch all transactions: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch all transactions: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} transactions (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, ServiceError> {
        if req.card_number.trim().is_empty() {
            return Err(ServiceError::Custom("Card number is required".to_string()));
        }

        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let masked_card = mask_card_number(&req.card_number);
        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "💳 Fetching transactions by card number: {} | Page: {page}, Size: {page_size}, Search: {:?}",
            masked_card, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_all_transactions_by_card_number",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_by_card_number"),
                KeyValue::new("card_number", masked_card.clone()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:find_by_card_number:card:{}:page:{page}:size:{page_size}:search:{}",
            masked_card,
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} transactions in cache for card: {}",
                cache.data.len(),
                masked_card
            );
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self.query.find_all_by_card_number(req).await {
            Ok(res) => {
                let log_msg = format!(
                    "✅ Found {} transactions for card {}",
                    res.0.len(),
                    masked_card
                );
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch transactions for card {masked_card}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch transactions for card: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Transactions by card number retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} transactions for card {} (total: {total_items})",
            response.data.len(),
            masked_card
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, ServiceError> {
        info!("🔍 Finding transaction by ID: {transaction_id}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_transaction_by_id",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("transaction_id", transaction_id.to_string()),
            ],
        );

        let mut request = Request::new(transaction_id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:find_by_id:id:{transaction_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<TransactionResponse>>(&cache_key)
            .await
        {
            info!("✅ Found transaction in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Transaction retrieved from cache")
                .await;
            return Ok(cache);
        }

        let transaction = match self.query.find_by_id(transaction_id).await {
            Ok(transaction) => {
                info!("✅ Found transaction with ID: {transaction_id}");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Transaction retrieved successfully",
                    )
                    .await;
                transaction
            }
            Err(e) => {
                error!("❌ Database error while finding transaction ID {transaction_id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Transaction retrieved successfully".to_string(),
            data: TransactionResponse::from(transaction),
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Found transaction: '{}' (ID: {transaction_id})",
            response.data.id
        );

        Ok(response)
    }

    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, ServiceError> {
        info!("🏢 Fetching transactions for merchant ID: {merchant_id}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_transactions_by_merchant_id",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_by_merchant_id"),
                KeyValue::new("merchant_id", merchant_id.to_string()),
            ],
        );

        let mut request = Request::new(merchant_id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("transaction:find_by_merchant_id:merchant_id:{merchant_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<TransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found transactions for merchant in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Transactions for merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        let transactions = match self.query.find_by_merchant_id(merchant_id).await {
            Ok(transactions) => {
                info!(
                    "✅ Found {} transactions for merchant ID {merchant_id}",
                    transactions.len()
                );
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Transactions for merchant retrieved successfully",
                    )
                    .await;
                transactions
            }
            Err(e) => {
                error!("❌ Failed to fetch transactions for merchant ID {merchant_id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to fetch transactions for merchant: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let transaction_responses: Vec<TransactionResponse> = transactions
            .into_iter()
            .map(TransactionResponse::from)
            .collect();

        let response = ApiResponse {
            status: "success".to_string(),
            message: "Transactions by merchant ID retrieved successfully".to_string(),
            data: transaction_responses,
        };

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        info!(
            "✅ Retrieved {} transactions for merchant ID {merchant_id}",
            response.data.len()
        );

        Ok(response)
    }

    async fn find_by_active(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🟢 Searching active transactions | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_active_transactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:find_by_active:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ Found {} active transactions in cache", cache.data.len());
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self.query.find_by_active(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} active transactions", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch active transactions: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch active transactions: {e:?}"),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponseDeleteAt> = transactions
            .into_iter()
            .map(TransactionResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Active transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "✅ Found {} active transactions (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }

    async fn find_by_trashed(
        &self,
        req: &FindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, ServiceError> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let search_str = search.clone().unwrap_or_else(|| "None".to_string());

        info!(
            "🗑️  Searching trashed transactions | Page: {}, Size: {}, Search: {:?}",
            page, page_size, search_str
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "find_trashed_transactions",
            vec![
                KeyValue::new("component", "transaction"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search_str.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!(
            "transaction:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            search.unwrap_or_default()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>>(&cache_key)
            .await
        {
            let log_msg = format!(
                "✅ Found {} trashed transactions in cache",
                cache.data.len()
            );
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let (transactions, total_items) = match self.query.find_by_trashed(req).await {
            Ok(res) => {
                let log_msg = format!("✅ Found {} trashed transactions", res.0.len());
                info!("{log_msg}");
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, &log_msg)
                    .await;
                res
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed transactions: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("❌ Failed to fetch trashed transactions: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let transaction_responses: Vec<TransactionResponseDeleteAt> = transactions
            .into_iter()
            .map(TransactionResponseDeleteAt::from)
            .collect();

        let response = ApiResponsePagination {
            status: "success".to_string(),
            message: "Trashed transactions retrieved successfully".to_string(),
            data: transaction_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        };

        info!(
            "🗑️  Found {} trashed transactions (total: {total_items})",
            response.data.len()
        );

        self.cache_store
            .set_to_cache(&cache_key, &response, Duration::minutes(10))
            .await;

        Ok(response)
    }
}
