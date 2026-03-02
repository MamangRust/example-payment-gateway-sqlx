use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::merchant::{
    CreateMerchantRequest, FindAllMerchantApikey, FindAllMerchantRequest,
    FindAllMerchantTransaction, FindByApiKeyRequest, FindByIdMerchantRequest,
    FindByMerchantUserIdRequest, FindYearMerchant, FindYearMerchantByApikey, FindYearMerchantById,
    UpdateMerchantRequest, merchant_service_client::MerchantServiceClient,
};
use opentelemetry::KeyValue;
use shared::utils::mask_api_key;
use shared::{
    abstract_trait::merchant::http::{
        MerchantCommandGrpcClientTrait, MerchantGrpcClientServiceTrait,
        MerchantQueryGrpcClientTrait, MerchantStatsAmountByApiKeyGrpcClientTrait,
        MerchantStatsAmountByMerchantGrpcClientTrait, MerchantStatsAmountGrpcClientTrait,
        MerchantStatsMethodByApiKeyGrpcClientTrait, MerchantStatsMethodByMerchantGrpcClientTrait,
        MerchantStatsMethodGrpcClientTrait, MerchantStatsTotalAmountByApiKeyGrpcClientTrait,
        MerchantStatsTotalAmountByMerchantGrpcClientTrait, MerchantStatsTotalAmountGrpcClientTrait,
        MerchantTransactionGrpcClientTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::merchant::{
            CreateMerchantRequest as DomainCreateMerchantRequest,
            FindAllMerchantTransactions as DomainFindAllMerchantTransactions,
            FindAllMerchantTransactionsByApiKey as DomainFindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById as DomainFindAllMerchantTransactionsById,
            FindAllMerchants as DomainFindAllMerchants,
            MonthYearAmountApiKey as DomainMonthYearAmountApiKey,
            MonthYearAmountMerchant as DomainMonthYearAmountMerchant,
            MonthYearPaymentMethodApiKey as DomainMonthYearPaymentMethodApiKey,
            MonthYearPaymentMethodMerchant as DomainMonthYearPaymentMethodMerchant,
            MonthYearTotalAmountApiKey as DomainMonthYearTotalAmountApiKey,
            MonthYearTotalAmountMerchant as DomainMonthYearTotalAmountMerchant,
            UpdateMerchantRequest as DomainUpdateMerchantRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
            MerchantResponseMonthlyAmount, MerchantResponseMonthlyPaymentMethod,
            MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyAmount,
            MerchantResponseYearlyPaymentMethod, MerchantResponseYearlyTotalAmount,
            MerchantTransactionResponse,
        },
    },
    errors::{AppErrorGrpc, HttpError},
    observability::{Method, TracingMetrics},
};
use std::sync::Arc;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

pub struct MerchantGrpcClientService {
    client: MerchantServiceClient<Channel>,
    tracing_metrics_core: TracingMetrics,
    cache_store: Arc<CacheStore>,
}

impl MerchantGrpcClientService {
    pub fn new(client: MerchantServiceClient<Channel>, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            client,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl MerchantGrpcClientServiceTrait for MerchantGrpcClientService {}

#[async_trait]
impl MerchantQueryGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;

        info!(
            "fetching all merchants - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindAllMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_all"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:find_all:page:{page}:size:{page_size}:search:{}",
            request.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchants in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Merchants retrieved from cache")
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_all_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchants",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponse> = inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} merchants", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to fetch merchants")
                    .await;
                error!("find_all merchants failed: {status:?}");

                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_active(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;

        info!(
            "fetching active merchants - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindActiveMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_active"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:find_by_active:page:{page}:size:{page_size}:search:{}",
            request.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found active merchants in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Active merchants retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_active(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched active merchants",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} active merchants", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch active merchants",
                    )
                    .await;
                error!("find_active merchants failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_trashed(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;

        info!(
            "fetching trashed merchants - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindTrashedMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_trashed"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantRequest {
            page,
            page_size,
            search: request.search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:find_by_trashed:page:{page}:size:{page_size}:search:{}",
            request.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>>(&cache_key)
            .await
        {
            info!("✅ Found trashed merchants in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Trashed merchants retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_trashed(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched trashed merchants",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!("fetched {} trashed merchants", api_response.data.len());

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch trashed merchants",
                    )
                    .await;
                error!("find_trashed merchants failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, HttpError> {
        info!("fetching merchant by api_key: *** (masked)");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindByApiKeyMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_apikey"),
            ],
        );

        let mut grpc_req = Request::new(FindByApiKeyRequest {
            api_key: api_key.to_string(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let masked_key = mask_api_key(api_key);

        let cache_key = format!("merchant:find_by_apikey:key:{masked_key}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<MerchantResponse>>(&cache_key)
            .await
        {
            info!("✅ Found merchant in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Merchant retrieved from cache")
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_api_key(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchant by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("merchant with api_key - data missing in gRPC response");
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let api_response = ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(15))
                    .await;

                info!("found merchant by api_key");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch merchant by api key",
                    )
                    .await;
                error!("find merchant by api_key failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, HttpError> {
        info!("fetching merchants by user_id: {user_id}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindByMerchantUserId",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_user_id"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByMerchantUserIdRequest { user_id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:find_by_user_id:user_id:{user_id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchants for user in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchants for user retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_merchant_user_id(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchants by user id",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponse> = inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} merchants for user_id {user_id}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch merchants by user id",
                    )
                    .await;
                error!("find merchants by user_id {user_id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, HttpError> {
        info!("fetching merchant by id: {id}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindByIdMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "find_by_id"),
                KeyValue::new("merchant_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:find_by_id:id:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<MerchantResponse>>(&cache_key)
            .await
        {
            info!("✅ Found merchant in cache");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "Merchant retrieved from cache")
                .await;
            return Ok(cache);
        }

        match self.client.clone().find_by_id_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchant by id",
                    )
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("merchant {id} - data missing in gRPC response");
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let api_response = ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(15))
                    .await;

                info!("found merchant {id}");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to fetch merchant by id")
                    .await;
                error!("find merchant {id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantTransactionGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions(
        &self,
        request: &DomainFindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;
        let search = &request.search;

        info!(
            "fetching all merchant transactions - page: {page}, page_size: {page_size}, search: {:?}",
            search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindAllMerchantTransactions",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_all_transactions"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantRequest {
            page,
            page_size,
            search: search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant_transaction:find_all:page:{page}:size:{page_size}:search:{:?}",
            search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_all_transaction_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched all merchant transactions",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };
                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch all merchant transactions",
                    )
                    .await;
                error!("fetch all merchant transactions failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions_by_api_key(
        &self,
        request: &DomainFindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;
        let search = &request.search;

        info!(
            "fetching merchant transactions by api_key: *** (masked) - page: {page}, page_size: {page_size}, search: {:?}",
            search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindAllMerchantTransactionsByApiKey",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_all_transactions_by_api_key"),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", search.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantApikey {
            api_key: request.api_key.clone(),
            page,
            page_size,
            search: search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let masked_key = mask_api_key(&request.api_key);

        let cache_key = format!(
            "merchant_transaction:find_by_api_key:key:{masked_key}:page:{page}:size:{page_size}:search:{}",
            search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant transactions retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_all_transaction_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchant transactions by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} merchant transactions for api_key",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch merchant transactions by api key",
                    )
                    .await;
                error!("fetch merchant transactions by api_key failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions_by_id(
        &self,
        request: &DomainFindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, HttpError> {
        let page = request.page;
        let page_size = request.page_size;
        let merchant_id = request.merchant_id;

        info!(
            "fetching merchant transactions for merchant_id: {merchant_id} - page: {page}, page_size: {page_size}, search: {:?}",
            request.search
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "FindAllMerchantTransactionsById",
            vec![
                KeyValue::new("component", "merchant_transaction"),
                KeyValue::new("operation", "find_all_transactions_by_id"),
                KeyValue::new("merchant_id", merchant_id.to_string()),
                KeyValue::new("page", page.to_string()),
                KeyValue::new("page_size", page_size.to_string()),
                KeyValue::new("search", request.search.clone()),
            ],
        );

        let mut grpc_req = Request::new(FindAllMerchantTransaction {
            merchant_id,
            page,
            page_size,
            search: request.search.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant_transaction:find_by_id:merchant_id:{merchant_id}:page:{page}:size:{page_size}:search:{}",
            request.search.clone()
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponsePagination<Vec<MerchantTransactionResponse>>>(&cache_key)
            .await
        {
            info!("✅ Found merchant transactions by ID in cache");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant transactions by ID retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_all_transaction_by_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched merchant transactions by id",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                let pagination = inner.pagination.map(Into::into).unwrap_or_default();

                let api_response = ApiResponsePagination {
                    data,
                    pagination,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
                    .await;

                info!(
                    "fetched {} transactions for merchant {}",
                    api_response.data.len(),
                    request.merchant_id
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch merchant transactions by id",
                    )
                    .await;
                error!(
                    "fetch transactions for merchant {} failed: {status:?}",
                    request.merchant_id
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantCommandGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: &DomainCreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, HttpError> {
        info!("creating merchant for user_id: {}", request.user_id);

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "CreateMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "create"),
                KeyValue::new("user_id", request.user_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(CreateMerchantRequest {
            name: request.name.clone(),
            user_id: request.user_id,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().create_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully created merchant")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "merchant creation failed - data missing in gRPC response for user_id: {}",
                        request.user_id
                    );
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let merchant_response: MerchantResponse = data.into();
                let masked_key = mask_api_key(&merchant_response.clone().api_key);

                let api_response = ApiResponse {
                    data: merchant_response.clone(),
                    status: inner.status,
                    message: inner.message,
                };

                let cache_key_delete = vec![
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key_delete in cache_key_delete {
                    self.cache_store.delete_from_cache(&key_delete).await;
                }

                let cache_key = [
                    format!("merchant:find_by_apikey:key:{masked_key}"),
                    format!("merchant:find_by_id:id:{}", merchant_response.clone().id),
                    format!(
                        "merchant:find_by_user_id:user_id:{}",
                        merchant_response.clone().user_id
                    ),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!(
                    "merchant created successfully for user_id: {}",
                    request.user_id
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to create merchant")
                    .await;
                error!(
                    "create merchant for user_id {} failed: {status:?}",
                    request.user_id
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: &DomainUpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, HttpError> {
        let merchant_id = request
            .merchant_id
            .ok_or_else(|| HttpError::Internal("merchant_id is required".to_string()))?;

        info!("updating merchant id: {merchant_id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "UpdateMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "update"),
                KeyValue::new("merchant_id", merchant_id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(UpdateMerchantRequest {
            merchant_id,
            user_id: request.user_id,
            name: request.name.clone(),
            status: request.status.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().update_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully updated merchant")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update merchant {merchant_id} - data missing in gRPC response",);
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let merchant_response: MerchantResponse = data.into();

                let api_response = ApiResponse {
                    data: merchant_response.clone(),
                    status: inner.status,
                    message: inner.message,
                };

                let masked_key = mask_api_key(&merchant_response.clone().api_key);

                let cache_key_delete = vec![
                    format!("merchant:find_by_id:id:{:?}", request.merchant_id),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key_delete in cache_key_delete {
                    self.cache_store.delete_from_cache(&key_delete).await;
                }

                let cache_key = [
                    format!("merchant:find_by_apikey:key:{}", masked_key),
                    format!("merchant:find_by_id:id:{:?}", request.merchant_id),
                    format!(
                        "merchant:find_by_user_id:user_id:{}",
                        merchant_response.clone().user_id
                    ),
                ];

                for key in cache_key {
                    self.cache_store
                        .set_to_cache(&key, &api_response, Duration::minutes(10))
                        .await;
                }

                info!("merchant {merchant_id} updated successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to update merchant")
                    .await;
                error!("update merchant {merchant_id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, HttpError> {
        info!("trashing merchant id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "TrashMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("merchant_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().trashed_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully trashed merchant")
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash merchant {id} - data missing in gRPC response");
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let merchant_response: MerchantResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: merchant_response,
                    status: inner.status,
                    message: inner.message,
                };

                let masked_key = mask_api_key(&api_response.data.api_key);

                let cache_keys = vec![
                    format!("merchant:find_by_id:id:{id}"),
                    format!(
                        "merchant:find_by_user_id:user_id:{}",
                        api_response.data.user_id
                    ),
                    format!("merchant:find_by_apikey:key:{masked_key}"),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("merchant {id} trashed successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to trash merchant")
                    .await;
                error!("trash merchant {id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, HttpError> {
        info!("restoring merchant id: {id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "RestoreMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("merchant_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully restored merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore merchant {id} - data missing in gRPC response");
                    HttpError::Internal("Merchant data is missing in gRPC response".into())
                })?;

                let merchant_response: MerchantResponseDeleteAt = data.into();

                let api_response = ApiResponse {
                    data: merchant_response,
                    status: inner.status,
                    message: inner.message,
                };

                let masked_key = mask_api_key(&api_response.data.api_key);

                let cache_keys = vec![
                    format!("merchant:find_by_id:id:{id}"),
                    format!(
                        "merchant:find_by_user_id:user_id:{}",
                        api_response.data.user_id
                    ),
                    format!("merchant:find_by_apikey:key:{masked_key}"),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("merchant {id} restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to restore merchant")
                    .await;
                error!("restore merchant {id} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting merchant id: {id}");

        let method = Method::Delete;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "DeleteMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("merchant_id", id.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_merchant_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully deleted merchant permanently",
                    )
                    .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    format!("merchant:find_by_id:id:{id}"),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("merchant {id} permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to delete merchant permanently",
                    )
                    .await;
                error!("delete merchant {id} permanently failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("restoring all trashed merchants");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "RestoreAllMerchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self.client.clone().restore_all_merchant(grpc_req).await {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully restored all trashed merchants",
                    )
                    .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "merchant:find_by_id:id:*".to_string(),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all trashed merchants restored successfully");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to restore all trashed merchants",
                    )
                    .await;
                error!("restore all merchants failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, HttpError> {
        info!("permanently deleting all merchants");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "DeleteAllMerchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut grpc_req = Request::new(());

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        match self
            .client
            .clone()
            .delete_all_merchant_permanent(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully deleted all merchants permanently",
                    )
                    .await;

                let inner = response.into_inner();

                let api_response = ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                };

                let cache_keys = vec![
                    "merchant:find_by_id:id:*".to_string(),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_by_active:*".to_string(),
                    "merchant:find_by_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                info!("all merchants permanently deleted");

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to delete all merchants permanently",
                    )
                    .await;
                error!("delete all merchants permanently failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsAmountGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, HttpError> {
        info!("fetching monthly AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyAmountMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:monthly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly merchant amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly merchant amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_amount_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly merchant amount",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly merchant amount",
                    )
                    .await;
                error!("fetch monthly AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, HttpError> {
        info!("fetching yearly AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyAmountMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:yearly:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly merchant amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly merchant amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_amount_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly merchant amount",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} yearly amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly merchant amount",
                    )
                    .await;
                error!("fetch yearly AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, HttpError> {
        info!("fetching monthly PAYMENT METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyMethodMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:monthly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly payment method statistics in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_payment_methods_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly payment method stats",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly payment method records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly payment method stats",
                    )
                    .await;
                error!("fetch monthly PAYMENT METHOD for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, HttpError> {
        info!("fetching yearly PAYMENT METHOD stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyMethodMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_method"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:yearly_method:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly payment method statistics in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_payment_method_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly payment method stats",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(2))
                    .await;

                info!(
                    "fetched {} yearly payment method records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly payment method stats",
                    )
                    .await;
                error!("fetch yearly PAYMENT METHOD for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, HttpError> {
        info!("fetching monthly TOTAL AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyTotalAmountMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_total_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:monthly_total:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found monthly total transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_total_amount_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly total amount stats",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly total amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly total amount stats",
                    )
                    .await;
                error!("fetch monthly TOTAL AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, HttpError> {
        info!("fetching yearly TOTAL AMOUNT stats for year: {year}");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyTotalAmountMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_total_amount"),
                KeyValue::new("year", year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchant { year });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!("merchant:yearly_total:year:{year}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!("✅ Found yearly total transaction amounts in cache for year: {year}");
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_total_amount_merchant(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly total amount stats",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly total amount records for year {year}",
                    api_response.data.len()
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly total amount stats",
                    )
                    .await;
                error!("fetch yearly TOTAL AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsAmountByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_bymerchant(
        &self,
        req: &DomainMonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, HttpError> {
        info!(
            "fetching monthly AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyAmountByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_amount_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_amount_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly amount by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly amount records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly amount by merchant",
                    )
                    .await;
                error!(
                    "fetch monthly AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_bymerchant(
        &self,
        req: &DomainMonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, HttpError> {
        info!(
            "fetching yearly AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyAmountByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_amount_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_amount_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly amount by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly amount records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly amount by merchant",
                    )
                    .await;
                error!(
                    "fetch yearly AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_bymerchant(
        &self,
        req: &DomainMonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, HttpError> {
        info!(
            "fetching monthly PAYMENT METHOD for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyMethodByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_method_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_method:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly payment method statistics in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_payment_method_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly payment method by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly payment method records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly payment method by merchant",
                    )
                    .await;
                error!(
                    "fetch monthly PAYMENT METHOD for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_bymerchant(
        &self,
        req: &DomainMonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, HttpError> {
        info!(
            "fetching yearly PAYMENT METHOD for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyMethodByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_method_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_method:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly payment method statistics in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_payment_method_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly payment method by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} yearly payment method records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly payment method by merchant",
                    )
                    .await;
                error!(
                    "fetch yearly PAYMENT METHOD for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_total_amount_bymerchant(
        &self,
        req: &DomainMonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, HttpError> {
        info!(
            "fetching monthly TOTAL AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyTotalAmountByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_total_amount_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_total_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_total_amount_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly total amount by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly total amount records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly total amount by merchant",
                    )
                    .await;
                error!(
                    "fetch monthly TOTAL AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_total_amount_bymerchant(
        &self,
        req: &DomainMonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, HttpError> {
        info!(
            "fetching yearly TOTAL AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyTotalAmountByMerchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_total_amount_bymerchant"),
                KeyValue::new("merchant_id", req.merchant_id.to_string()),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_total_amount:merchant_id:{}:year:{}",
            req.merchant_id, req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly total transaction amounts in cache for merchant_id: {}",
                req.merchant_id
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts by merchant retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_total_amount_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly total amount by merchant",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly total amount records for merchant {} year {}",
                    api_response.data.len(),
                    req.merchant_id,
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly total amount by merchant",
                    )
                    .await;
                error!(
                    "fetch yearly TOTAL AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsAmountByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_byapikey(
        &self,
        req: &DomainMonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, HttpError> {
        info!(
            "fetching monthly AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyAmountByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_amount_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_amount_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly amount by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} monthly amount records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly amount by api key",
                    )
                    .await;
                error!(
                    "fetch monthly AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_byapikey(
        &self,
        req: &DomainMonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, HttpError> {
        info!(
            "fetching yearly AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyAmountByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_amount_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_amount_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly amount by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} yearly amount records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly amount by api key",
                    )
                    .await;
                error!(
                    "fetch yearly AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_byapikey(
        &self,
        req: &DomainMonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, HttpError> {
        info!(
            "fetching monthly PAYMENT METHOD by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyMethodByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_method_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_method:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly payment method statistics in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly payment method statistics by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_payment_method_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly payment method by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly payment method records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );
                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly payment method by api key",
                    )
                    .await;
                error!(
                    "fetch monthly PAYMENT METHOD by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_byapikey(
        &self,
        req: &DomainMonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, HttpError> {
        info!(
            "fetching yearly PAYMENT METHOD by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyMethodByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_method_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_method:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly payment method statistics in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly payment method statistics by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_payment_method_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly payment method by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                info!(
                    "fetched {} yearly payment method records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly payment method by api key",
                    )
                    .await;
                error!(
                    "fetch yearly PAYMENT METHOD by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_total_amount_byapikey(
        &self,
        req: &DomainMonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, HttpError> {
        info!(
            "fetching monthly TOTAL AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMonthlyTotalAmountByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_monthly_total_amount_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:monthly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found monthly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Monthly total transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_monthly_total_amount_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched monthly total amount by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::minutes(30))
                    .await;

                info!(
                    "fetched {} monthly total amount records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch monthly total amount by api key",
                    )
                    .await;
                error!(
                    "fetch monthly TOTAL AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_total_amount_byapikey(
        &self,
        req: &DomainMonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, HttpError> {
        info!(
            "fetching yearly TOTAL AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetYearlyTotalAmountByApiKey",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "get_yearly_total_amount_byapikey"),
                KeyValue::new("year", req.year.to_string()),
            ],
        );

        let mut grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut grpc_req);

        let cache_key = format!(
            "merchant:yearly_total_amount:api_key:{}:year:{}",
            mask_api_key(&req.api_key),
            req.year
        );

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>>(&cache_key)
            .await
        {
            info!(
                "✅ Found yearly total transaction amounts in cache for api_key: {}",
                mask_api_key(&req.api_key)
            );
            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Yearly total transaction amounts by API key retrieved from cache",
                )
                .await;
            return Ok(cache);
        }

        match self
            .client
            .clone()
            .find_yearly_total_amount_by_apikey(grpc_req)
            .await
        {
            Ok(response) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched yearly total amount by api key",
                    )
                    .await;

                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                let api_response = ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                };

                self.cache_store
                    .set_to_cache(&cache_key, &api_response, Duration::hours(1))
                    .await;

                info!(
                    "fetched {} yearly total amount records for api_key year {}",
                    api_response.data.len(),
                    req.year
                );

                Ok(api_response)
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method,
                        "Failed to fetch yearly total amount by api key",
                    )
                    .await;
                error!(
                    "fetch yearly TOTAL AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(status).into())
            }
        }
    }
}
