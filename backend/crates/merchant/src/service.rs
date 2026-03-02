use crate::state::AppState;
use genproto::merchant::{
    ApiResponseMerchant, ApiResponseMerchantAll, ApiResponseMerchantDelete,
    ApiResponseMerchantDeleteAt, ApiResponseMerchantMonthlyAmount,
    ApiResponseMerchantMonthlyPaymentMethod, ApiResponseMerchantMonthlyTotalAmount,
    ApiResponseMerchantYearlyAmount, ApiResponseMerchantYearlyPaymentMethod,
    ApiResponseMerchantYearlyTotalAmount, ApiResponsePaginationMerchant,
    ApiResponsePaginationMerchantDeleteAt, ApiResponsePaginationMerchantTransaction,
    ApiResponsesMerchant, CreateMerchantRequest, FindAllMerchantApikey, FindAllMerchantRequest,
    FindAllMerchantTransaction, FindByApiKeyRequest, FindByIdMerchantRequest,
    FindByMerchantUserIdRequest, FindYearMerchant, FindYearMerchantByApikey, FindYearMerchantById,
    UpdateMerchantRequest, merchant_service_server::MerchantService,
};
use shared::{
    domain::requests::merchant::{
        CreateMerchantRequest as DomainCreateMerchantRequest, FindAllMerchantTransactions,
        FindAllMerchantTransactionsByApiKey, FindAllMerchantTransactionsById, FindAllMerchants,
        MonthYearAmountApiKey, MonthYearAmountMerchant, MonthYearPaymentMethodApiKey,
        MonthYearPaymentMethodMerchant, MonthYearTotalAmountApiKey, MonthYearTotalAmountMerchant,
        UpdateMerchantRequest as DomainUpdateMerchantRequest,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct MerchantServiceImpl {
    pub state: Arc<AppState>,
}

impl MerchantServiceImpl {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    async fn check_rate_limit(&self) -> Result<(), Status> {
        self.state.load_monitor.record_request();

        if self.state.circuit_breaker.is_open() {
            warn!("Request rejected: circuit breaker open");
            return Err(Status::unavailable(
                "Service temporarily unavailable due to high error rate. Please try again later.",
            ));
        }

        match self.state.di_container.request_limiter.try_acquire() {
            Ok(_permit) => Ok(()),
            Err(_) => {
                warn!("Request rejected: rate limit exceeded");
                Err(Status::resource_exhausted(
                    "Too many concurrent requests. Please try again later.",
                ))
            }
        }
    }
}

#[tonic::async_trait]
impl MerchantService for MerchantServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_merchant",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_merchant(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchant {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_all_merchant success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_merchant failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_merchant", merchant_id = request.get_ref().merchant_id))]
    async fn find_by_id_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_by_id(merchant_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(merchant_id = merchant_id, "find_by_id_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_by_id_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_by_id_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_api_key", api_key = %request.get_ref().api_key))]
    async fn find_by_api_key(
        &self,
        request: Request<FindByApiKeyRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_by_apikey(&api_key)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(api_key = api_key, "find_by_api_key success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_by_api_key rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_by_api_key failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_transaction_merchant",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_transaction_merchant(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllMerchantTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_transaction
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchantTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_all_transaction_merchant success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_transaction_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_transaction_merchant failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_payment_methods_merchant", year = request.get_ref().year))]
    async fn find_monthly_payment_methods_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method
                    .get_monthly_method(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_payment_methods_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_payment_methods_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_payment_methods_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_payment_method_merchant", year = request.get_ref().year))]
    async fn find_yearly_payment_method_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method
                    .get_yearly_method(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_payment_method_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_payment_method_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_payment_method_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_amount_merchant", year = request.get_ref().year))]
    async fn find_monthly_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount
                    .get_monthly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_amount_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_amount_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_amount_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_amount_merchant", year = request.get_ref().year))]
    async fn find_yearly_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount
                    .get_yearly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_amount_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_amount_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_amount_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_total_amount_merchant", year = request.get_ref().year))]
    async fn find_monthly_total_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount
                    .get_monthly_total_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_total_amount_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_total_amount_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_total_amount_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_total_amount_merchant", year = request.get_ref().year))]
    async fn find_yearly_total_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount
                    .get_yearly_total_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_total_amount_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_total_amount_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_total_amount_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_transaction_by_merchant",
        merchant_id = request.get_ref().merchant_id,
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_transaction_by_merchant(
        &self,
        request: Request<FindAllMerchantTransaction>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = FindAllMerchantTransactionsById {
            merchant_id,
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_transaction
                    .find_all_by_id(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchantTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    "find_all_transaction_by_merchant success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_all_transaction_by_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_all_transaction_by_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_payment_method_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_monthly_payment_method_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearPaymentMethodMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method_by_merchant
                    .find_monthly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_monthly_payment_method_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_monthly_payment_method_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_monthly_payment_method_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_payment_method_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_yearly_payment_method_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearPaymentMethodMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method_by_merchant
                    .find_yearly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_yearly_payment_method_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_yearly_payment_method_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_yearly_payment_method_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_amount_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_monthly_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearAmountMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount_by_merchant
                    .find_monthly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_monthly_amount_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_monthly_amount_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_monthly_amount_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_amount_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_yearly_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearAmountMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount_by_merchant
                    .find_yearly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_yearly_amount_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_yearly_amount_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_yearly_amount_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_total_amount_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_monthly_total_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearTotalAmountMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount_by_merchant
                    .find_monthly_total_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_monthly_total_amount_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_monthly_total_amount_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_monthly_total_amount_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_total_amount_by_merchants",
        merchant_id = request.get_ref().merchant_id,
        year = request.get_ref().year
    ))]
    async fn find_yearly_total_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = MonthYearTotalAmountMerchant {
            merchant_id,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount_by_merchant
                    .find_yearly_total_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    year = domain_req.year,
                    "find_yearly_total_amount_by_merchants success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_yearly_total_amount_by_merchants rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_yearly_total_amount_by_merchants failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_transaction_by_apikey",
        api_key = %request.get_ref().api_key,
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_transaction_by_apikey(
        &self,
        request: Request<FindAllMerchantApikey>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = FindAllMerchantTransactionsByApiKey {
            api_key: api_key.clone(),
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_transaction
                    .find_all_by_api_key(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchantTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(api_key = api_key, "find_all_transaction_by_apikey success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_all_transaction_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_all_transaction_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_payment_method_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_monthly_payment_method_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearPaymentMethodApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method_by_apikey
                    .find_monthly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    api_key = api_key,
                    "find_monthly_payment_method_by_apikey success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_monthly_payment_method_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_monthly_payment_method_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_payment_method_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_yearly_payment_method_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearPaymentMethodApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_method_by_apikey
                    .find_yearly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    api_key = api_key,
                    "find_yearly_payment_method_by_apikey success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_yearly_payment_method_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_yearly_payment_method_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_amount_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_monthly_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearAmountApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount_by_apikey
                    .find_monthly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(api_key = api_key, "find_monthly_amount_by_apikey success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_monthly_amount_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_monthly_amount_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_amount_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_yearly_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearAmountApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_amount_by_apikey
                    .find_yearly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(api_key = api_key, "find_yearly_amount_by_apikey success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_yearly_amount_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_yearly_amount_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_total_amount_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_monthly_total_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearTotalAmountApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount_by_apikey
                    .find_monthly_total_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    api_key = api_key,
                    "find_monthly_total_amount_by_apikey success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_monthly_total_amount_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_monthly_total_amount_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_total_amount_by_apikey",
        api_key = %request.get_ref().api_key,
        year = request.get_ref().year
    ))]
    async fn find_yearly_total_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let api_key = req.api_key.clone();
        let domain_req = MonthYearTotalAmountApiKey {
            api_key: api_key.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_stats_total_amount_by_apikey
                    .find_yearly_total_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    api_key = api_key,
                    "find_yearly_total_amount_by_apikey success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            api_key = api_key,
                            "find_yearly_total_amount_by_apikey rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(api_key = api_key, error = %inner, "find_yearly_total_amount_by_apikey failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_merchant_user_id", user_id = request.get_ref().user_id))]
    async fn find_by_merchant_user_id(
        &self,
        request: Request<FindByMerchantUserIdRequest>,
    ) -> Result<Response<ApiResponsesMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.user_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_merchant_user_id(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsesMerchant {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "find_by_merchant_user_id success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "find_by_merchant_user_id rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "find_by_merchant_user_id failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_active",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_active(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchantDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_by_active success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_trashed",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_query
                    .find_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationMerchantDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_by_trashed success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "create_merchant", user_id = request.get_ref().user_id))]
    async fn create_merchant(
        &self,
        request: Request<CreateMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.user_id;
        let domain_req = DomainCreateMerchantRequest {
            user_id: req.user_id,
            name: req.name,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "create_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "create_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "create_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "update_merchant", merchant_id = request.get_ref().merchant_id))]
    async fn update_merchant(
        &self,
        request: Request<UpdateMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;
        let domain_req = DomainUpdateMerchantRequest {
            merchant_id: Some(req.merchant_id),
            user_id: req.user_id,
            name: req.name,
            status: "pending".to_string(), // Sesuai dengan kode asli
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(merchant_id = merchant_id, "update_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "update_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "update_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_merchant", merchant_id = request.get_ref().merchant_id))]
    async fn trashed_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .trash(merchant_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(merchant_id = merchant_id, "trashed_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "trashed_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "trashed_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_merchant", merchant_id = request.get_ref().merchant_id))]
    async fn restore_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .restore(merchant_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(merchant_id = merchant_id, "restore_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "restore_merchant rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "restore_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_merchant_permanent", merchant_id = request.get_ref().merchant_id))]
    async fn delete_merchant_permanent(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let merchant_id = req.merchant_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .delete(merchant_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    merchant_id = merchant_id,
                    "delete_merchant_permanent success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "delete_merchant_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "delete_merchant_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_merchant"))]
    async fn restore_all_merchant(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseMerchantAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_merchant success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_merchant rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_merchant failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_merchant_permanent"))]
    async fn delete_all_merchant_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseMerchantAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .merchant_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMerchantAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_merchant_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_merchant_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_merchant_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
