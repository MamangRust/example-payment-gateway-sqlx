use crate::state::AppState;
use genproto::topup::{
    ApiResponsePaginationTopup, ApiResponsePaginationTopupDeleteAt, ApiResponseTopup,
    ApiResponseTopupAll, ApiResponseTopupDelete, ApiResponseTopupDeleteAt,
    ApiResponseTopupMonthAmount, ApiResponseTopupMonthMethod, ApiResponseTopupMonthStatusFailed,
    ApiResponseTopupMonthStatusSuccess, ApiResponseTopupYearAmount, ApiResponseTopupYearMethod,
    ApiResponseTopupYearStatusFailed, ApiResponseTopupYearStatusSuccess, ApiResponsesTopup,
    CreateTopupRequest, FindAllTopupByCardNumberRequest, FindAllTopupRequest,
    FindByCardNumberTopupRequest, FindByIdTopupRequest, FindMonthlyTopupStatus,
    FindMonthlyTopupStatusCardNumber, FindYearTopupCardNumber, FindYearTopupStatus,
    FindYearTopupStatusCardNumber, UpdateTopupRequest, topup_service_server::TopupService,
};
use shared::{
    domain::requests::topup::{
        CreateTopupRequest as DomainCreateTopupRequest, FindAllTopups, FindAllTopupsByCardNumber,
        MonthTopupStatus, MonthTopupStatusCardNumber,
        UpdateTopupRequest as DomainUpdateTopupRequst, YearMonthMethod, YearTopupStatusCardNumber,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::{mask_api_key, mask_card_number},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct TopupServiceImpl {
    pub state: Arc<AppState>,
}

impl TopupServiceImpl {
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
impl TopupService for TopupServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_topup",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all_topup(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTopups {
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
                    .topup_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTopup {
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
                    "find_all_topup success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_topup failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_topup_by_card_number",
        card_number = tracing::field::Empty,
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all_topup_by_card_number(
        &self,
        request: Request<FindAllTopupByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_api_key(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = FindAllTopupsByCardNumber {
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
                    .topup_query
                    .find_all_by_card_number(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTopup {
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
                    "find_all_topup_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_all_topup_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_all_topup_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_topup", topup_id = request.get_ref().topup_id), level = "info")]
    async fn find_by_id_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let topup_id = req.topup_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_query
                    .find_by_id(topup_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(topup_id = topup_id, "find_by_id_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            topup_id = topup_id,
                            "find_by_id_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(topup_id = topup_id, error = %inner, "find_by_id_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_status_success",
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_topup_status_success(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthTopupStatus {
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
                    .topup_stats_status
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    year = domain_req.year,
                    month = domain_req.month,
                    "find_monthly_topup_status_success success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = domain_req.year,
                            month = domain_req.month,
                            "find_monthly_topup_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = domain_req.year,
                            month = domain_req.month,
                            error = %inner,
                            "find_monthly_topup_status_success failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_status_success",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_status_success(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
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
                    .topup_stats_status
                    .get_yearly_status_success(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_topup_status_success success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_topup_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_topup_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_status_failed",
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_topup_status_failed(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthTopupStatus {
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
                    .topup_stats_status
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    year = domain_req.year,
                    month = domain_req.month,
                    "find_monthly_topup_status_failed success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = domain_req.year,
                            month = domain_req.month,
                            "find_monthly_topup_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = domain_req.year,
                            month = domain_req.month,
                            error = %inner,
                            "find_monthly_topup_status_failed failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_status_failed",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_status_failed(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
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
                    .topup_stats_status
                    .get_yearly_status_failed(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_topup_status_failed success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_topup_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_topup_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_topup_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthTopupStatusCardNumber {
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
                    .topup_stats_status_by_card
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthStatusSuccess {
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
                    "find_monthly_topup_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_topup_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_topup_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_status_success_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearTopupStatusCardNumber {
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
                    .topup_stats_status_by_card
                    .get_yearly_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearStatusSuccess {
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
                    "find_yearly_topup_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_topup_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_topup_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = MonthTopupStatusCardNumber {
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
                    .topup_stats_status_by_card
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthStatusFailed {
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
                    "find_monthly_topup_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_topup_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_topup_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearTopupStatusCardNumber {
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
                    .topup_stats_status_by_card
                    .get_yearly_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearStatusFailed {
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
                    "find_yearly_topup_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_topup_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_topup_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_methods",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
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
                    .topup_stats_method
                    .get_monthly_methods(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_topup_methods success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_topup_methods rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_topup_methods failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_methods",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
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
                    .topup_stats_method
                    .get_yearly_methods(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_topup_methods success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_topup_methods rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_topup_methods failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_amounts",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
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
                    .topup_stats_amount
                    .get_monthly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_topup_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_topup_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_topup_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_amounts",
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
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
                    .topup_stats_amount
                    .get_yearly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_topup_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_topup_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_topup_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_methods_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearMonthMethod {
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
                    .topup_stats_method_by_card
                    .get_monthly_methods(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthMethod {
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
                    "find_monthly_topup_methods_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_topup_methods_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_topup_methods_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_methods_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearMonthMethod {
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
                    .topup_stats_method_by_card
                    .get_yearly_methods(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearMethod {
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
                    "find_yearly_topup_methods_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_topup_methods_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_topup_methods_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_amounts_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_monthly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearMonthMethod {
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
                    .topup_stats_amount_by_card
                    .get_monthly_amounts(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupMonthAmount {
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
                    "find_monthly_topup_amounts_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_topup_amounts_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_topup_amounts_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_amounts_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ), level = "info")]
    async fn find_yearly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = YearMonthMethod {
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
                    .topup_stats_amount_by_card
                    .get_yearly_amounts(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupYearAmount {
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
                    "find_yearly_topup_amounts_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_topup_amounts_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_topup_amounts_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_card_number_topup",
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn find_by_card_number_topup(
        &self,
        request: Request<FindByCardNumberTopupRequest>,
    ) -> Result<Response<ApiResponsesTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_query
                    .find_by_card(&card_number)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsesTopup {
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
                    "find_by_card_number_topup success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_by_card_number_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "find_by_card_number_topup failed");
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
    ), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTopups {
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
                    .topup_query
                    .find_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTopupDeleteAt {
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
    ), level = "info")]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTopups {
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
                    .topup_query
                    .find_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTopupDeleteAt {
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
    #[instrument(skip(self, request), fields(
        method = "create_topup",
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn create_topup(
        &self,
        request: Request<CreateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = DomainCreateTopupRequest {
            card_number,
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = masked_card, "create_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "create_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "create_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "update_topup", topup_id = request.get_ref().topup_id), level = "info")]
    async fn update_topup(
        &self,
        request: Request<UpdateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let topup_id = req.topup_id;

        let domain_req = DomainUpdateTopupRequst {
            card_number: req.card_number,
            topup_id: Some(req.topup_id),
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(topup_id = topup_id, "update_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            topup_id = topup_id,
                            "update_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(topup_id = topup_id, error = %inner, "update_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_topup", topup_id = request.get_ref().topup_id), level = "info")]
    async fn trashed_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let topup_id = req.topup_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .trashed(topup_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(topup_id = topup_id, "trashed_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            topup_id = topup_id,
                            "trashed_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(topup_id = topup_id, error = %inner, "trashed_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_topup", topup_id = request.get_ref().topup_id), level = "info")]
    async fn restore_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let topup_id = req.topup_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .restore(topup_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(topup_id = topup_id, "restore_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            topup_id = topup_id,
                            "restore_topup rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(topup_id = topup_id, error = %inner, "restore_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_topup_permanent", topup_id = request.get_ref().topup_id), level = "info")]
    async fn delete_topup_permanent(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let topup_id = req.topup_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .delete_permanent(topup_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(topup_id = topup_id, "delete_topup_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            topup_id = topup_id,
                            "delete_topup_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(topup_id = topup_id, error = %inner, "delete_topup_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "restore_all_topup"),
        level = "info"
    )]
    async fn restore_all_topup(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_topup success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_topup rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_topup failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "delete_all_topup_permanent"),
        level = "info"
    )]
    async fn delete_all_topup_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .topup_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_topup_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_topup_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_topup_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
