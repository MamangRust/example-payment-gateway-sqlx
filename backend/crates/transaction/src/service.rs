use genproto::transaction::{
    ApiResponsePaginationTransaction, ApiResponsePaginationTransactionDeleteAt,
    ApiResponseTransaction, ApiResponseTransactionAll, ApiResponseTransactionDelete,
    ApiResponseTransactionDeleteAt, ApiResponseTransactionMonthAmount,
    ApiResponseTransactionMonthMethod, ApiResponseTransactionMonthStatusFailed,
    ApiResponseTransactionMonthStatusSuccess, ApiResponseTransactionYearAmount,
    ApiResponseTransactionYearMethod, ApiResponseTransactionYearStatusFailed,
    ApiResponseTransactionYearStatusSuccess, ApiResponseTransactions, CreateTransactionRequest,
    FindAllTransactionCardNumberRequest, FindAllTransactionRequest, FindByIdTransactionRequest,
    FindByYearCardNumberTransactionRequest, FindMonthlyTransactionStatus,
    FindMonthlyTransactionStatusCardNumber, FindTransactionByMerchantIdRequest,
    FindYearTransactionStatus, FindYearTransactionStatusCardNumber, UpdateTransactionRequest,
    transaction_service_server::TransactionService,
};
use std::sync::Arc;

use crate::state::AppState;
use shared::{
    domain::requests::transaction::{
        CreateTransactionRequest as DomainCreateTransactionRequest, FindAllTransactionCardNumber,
        FindAllTransactions, MonthStatusTransaction, MonthStatusTransactionCardNumber,
        MonthYearPaymentMethod, UpdateTransactionRequest as DomainUpdateTransactionRequest,
        YearStatusTransactionCardNumber,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::{mask_card_number, timestamp_to_naive_datetime},
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct TransactionServiceImpl {
    pub state: Arc<AppState>,
}

impl TransactionServiceImpl {
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
impl TransactionService for TransactionServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_transaction",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransactions {
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
                    .transaction_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransaction {
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
                    "find_all_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_transaction failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_transaction_by_card_number",
        card_number = tracing::field::Empty,
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all_transaction_by_card_number(
        &self,
        request: Request<FindAllTransactionCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = FindAllTransactionCardNumber {
            card_number,
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
                    .transaction_query
                    .find_all_by_card_number(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransaction {
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
                    card_number = masked_card,
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_all_transaction_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_all_transaction_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_all_transaction_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_transaction", transaction_id = request.get_ref().transaction_id), level = "info")]
    async fn find_by_id_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transaction_id = req.transaction_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_query
                    .find_by_id(transaction_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transaction_id = transaction_id,
                    "find_by_id_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transaction_id = transaction_id,
                            "find_by_id_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transaction_id = transaction_id, error = %inner, "find_by_id_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(
        method = "find_monthly_transaction_status_success",
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_transaction_status_success(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransaction { year, month };

                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    year = year,
                    month = month,
                    "find_monthly_transaction_status_success success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_transaction_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_transaction_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transaction_status_success",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_transaction_status_success(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
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
                    .transaction_stats_status
                    .get_yearly_status_success(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    year = year,
                    "find_yearly_transaction_status_success success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transaction_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_transaction_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transaction_status_failed",
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_transaction_status_failed(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransaction { year, month };

                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    year = year,
                    month = month,
                    "find_monthly_transaction_status_failed success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_transaction_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_transaction_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transaction_status_failed",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_transaction_status_failed(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
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
                    .transaction_stats_status
                    .get_yearly_status_failed(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transaction_status_failed success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transaction_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_transaction_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transaction_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthStatusTransactionCardNumber {
            card_number,
            year: req.year,
            month: req.month,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status_by_card
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    month = domain_req.month,
                    "find_monthly_transaction_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transaction_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transaction_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transaction_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearStatusTransactionCardNumber {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status_by_card
                    .get_yearly_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_yearly_transaction_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transaction_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transaction_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transaction_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthStatusTransactionCardNumber {
            card_number,
            year: req.year,
            month: req.month,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status_by_card
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    month = domain_req.month,
                    "find_monthly_transaction_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transaction_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transaction_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transaction_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearStatusTransactionCardNumber {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_status_by_card
                    .get_yearly_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_yearly_transaction_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transaction_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transaction_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_payment_methods",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
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
                    .transaction_stats_method
                    .get_monthly_method(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_payment_methods success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_payment_methods rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_payment_methods failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_payment_methods",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
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
                    .transaction_stats_method
                    .get_yearly_method(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_payment_methods success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_payment_methods rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_payment_methods failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_amounts",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
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
                    .transaction_stats_amount
                    .get_monthly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_amounts",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
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
                    .transaction_stats_amount
                    .get_yearly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_payment_methods_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthYearPaymentMethod {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_method_by_card
                    .get_monthly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_monthly_payment_methods_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_payment_methods_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_payment_methods_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_payment_methods_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthYearPaymentMethod {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_method_by_card
                    .get_yearly_method(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_yearly_payment_methods_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_payment_methods_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_payment_methods_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_amounts_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthYearPaymentMethod {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_amount_by_card
                    .get_monthly_amounts(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_monthly_amounts_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_amounts_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_amounts_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_amounts_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthYearPaymentMethod {
            card_number,
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_stats_amount_by_card
                    .get_yearly_amounts(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = masked_card,
                    year = domain_req.year,
                    "find_yearly_amounts_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_amounts_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_amounts_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_transaction_by_merchant_id", merchant_id = request.get_ref().merchant_id), level = "info")]
    async fn find_transaction_by_merchant_id(
        &self,
        request: Request<FindTransactionByMerchantIdRequest>,
    ) -> Result<Response<ApiResponseTransactions>, Status> {
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
                    .transaction_query
                    .find_by_merchant_id(merchant_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactions {
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
                    "find_transaction_by_merchant_id success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            merchant_id = merchant_id,
                            "find_transaction_by_merchant_id rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(merchant_id = merchant_id, error = %inner, "find_transaction_by_merchant_id failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_active_transaction",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_by_active_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransactions {
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
                    .transaction_query
                    .find_by_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransactionDeleteAt {
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
                    "find_by_active_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active_transaction failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_trashed_transaction",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_by_trashed_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransactions {
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
                    .transaction_query
                    .find_by_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransactionDeleteAt {
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
                    "find_by_trashed_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed_transaction failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "create_transaction",
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn create_transaction(
        &self,
        request: Request<CreateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);
        let api_key = req.api_key.clone();

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("transaction_time invalid"))?;

        let domain_req = DomainCreateTransactionRequest {
            card_number,
            amount: req.amount,
            payment_method: req.payment_method,
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .create(&api_key, &domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = masked_card, "create_transaction success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "create_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "create_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_transaction",
        transaction_id = request.get_ref().transaction_id,
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn update_transaction(
        &self,
        request: Request<UpdateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transaction_id = req.transaction_id;
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);
        let api_key = req.api_key.clone();

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("transaction_time invalid"))?;

        let domain_req = DomainUpdateTransactionRequest {
            transaction_id: Some(req.transaction_id),
            card_number,
            amount: req.amount as i64,
            payment_method: req.payment_method,
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .update(&api_key, &domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transaction_id = transaction_id,
                    card_number = masked_card,
                    "update_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transaction_id = transaction_id,
                            "update_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transaction_id = transaction_id, error = %inner, "update_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_transaction", transaction_id = request.get_ref().transaction_id), level = "info")]
    async fn trashed_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transaction_id = req.transaction_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .trashed(transaction_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transaction_id = transaction_id,
                    "trashed_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transaction_id = transaction_id,
                            "trashed_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transaction_id = transaction_id, error = %inner, "trashed_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_transaction", transaction_id = request.get_ref().transaction_id), level = "info")]
    async fn restore_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transaction_id = req.transaction_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .restore(transaction_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transaction_id = transaction_id,
                    "restore_transaction success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transaction_id = transaction_id,
                            "restore_transaction rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transaction_id = transaction_id, error = %inner, "restore_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_transaction_permanent", transaction_id = request.get_ref().transaction_id), level = "info")]
    async fn delete_transaction_permanent(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transaction_id = req.transaction_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .delete_permanent(transaction_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transaction_id = transaction_id,
                    "delete_transaction_permanent success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transaction_id = transaction_id,
                            "delete_transaction_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transaction_id = transaction_id, error = %inner, "delete_transaction_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "restore_all_transaction"),
        level = "info"
    )]
    async fn restore_all_transaction(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_transaction success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_transaction rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_transaction failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "delete_all_transaction_permanent"),
        level = "info"
    )]
    async fn delete_all_transaction_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transaction_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_transaction_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_transaction_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_transaction_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
