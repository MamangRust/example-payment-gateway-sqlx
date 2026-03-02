use crate::state::AppState;
use genproto::{
    card::FindByCardNumberRequest,
    withdraw::{
        ApiResponsePaginationWithdraw, ApiResponsePaginationWithdrawDeleteAt, ApiResponseWithdraw,
        ApiResponseWithdrawAll, ApiResponseWithdrawDelete, ApiResponseWithdrawDeleteAt,
        ApiResponseWithdrawMonthAmount, ApiResponseWithdrawMonthStatusFailed,
        ApiResponseWithdrawMonthStatusSuccess, ApiResponseWithdrawYearAmount,
        ApiResponseWithdrawYearStatusFailed, ApiResponseWithdrawYearStatusSuccess,
        ApiResponsesWithdraw, CreateWithdrawRequest, FindAllWithdrawByCardNumberRequest,
        FindAllWithdrawRequest, FindByIdWithdrawRequest, FindMonthlyWithdrawStatus,
        FindMonthlyWithdrawStatusCardNumber, FindYearWithdrawCardNumber, FindYearWithdrawStatus,
        FindYearWithdrawStatusCardNumber, UpdateWithdrawRequest,
        withdraw_service_server::WithdrawService,
    },
};
use shared::{
    domain::requests::withdraw::{
        CreateWithdrawRequest as DomainCreateWithdrawRequest, FindAllWithdrawCardNumber,
        FindAllWithdraws, MonthStatusWithdraw, MonthStatusWithdrawCardNumber,
        UpdateWithdrawRequest as DomainUpdateWithdrawRequest, YearMonthCardNumber,
        YearStatusWithdrawCardNumber,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::{mask_card_number, timestamp_to_naive_datetime},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct WithdrawServiceImpl {
    pub state: Arc<AppState>,
}

impl WithdrawServiceImpl {
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
impl WithdrawService for WithdrawServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_withdraw",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_withdraw(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
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
                    .withdraw_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationWithdraw {
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
                    "find_all_withdraw success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_withdraw failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_all_withdraw_by_card_number",
        card_number = tracing::field::Empty,
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_withdraw_by_card_number(
        &self,
        request: Request<FindAllWithdrawByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = FindAllWithdrawCardNumber {
            card_number,
            search: req.search.clone(),
            page: req.page,
            page_size: req.page_size,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_query
                    .find_all_by_card_number(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationWithdraw {
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
                    "find_all_withdraw_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_all_withdraw_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_all_withdraw_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_withdraw", withdraw_id = request.get_ref().withdraw_id))]
    async fn find_by_id_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let withdraw_id = req.withdraw_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_query
                    .find_by_id(withdraw_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(withdraw_id = withdraw_id, "find_by_id_withdraw success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            withdraw_id = withdraw_id,
                            "find_by_id_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(withdraw_id = withdraw_id, error = %inner, "find_by_id_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraw_status_success",
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_withdraw_status_success(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusWithdraw { year, month };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthStatusSuccess {
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
                    "find_monthly_withdraw_status_success success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_withdraw_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_withdraw_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraw_status_success",
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraw_status_success(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
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
                    .withdraw_stats_status
                    .get_yearly_status_success(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_withdraw_status_success success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_withdraw_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_withdraw_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraw_status_failed",
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_withdraw_status_failed(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusWithdraw { year, month };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthStatusFailed {
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
                    "find_monthly_withdraw_status_failed success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_withdraw_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_withdraw_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraw_status_failed",
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraw_status_failed(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
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
                    .withdraw_stats_status
                    .get_yearly_status_failed(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_withdraw_status_failed success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_withdraw_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_withdraw_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraw_status_success_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_withdraw_status_success_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusWithdrawCardNumber {
                    card_number,
                    year: req.year,
                    month: req.month,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status_by_card
                    .get_month_status_success_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthStatusSuccess {
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
                    year = req.year,
                    month = req.month,
                    "find_monthly_withdraw_status_success_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_withdraw_status_success_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_withdraw_status_success_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraw_status_success_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraw_status_success_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearStatusWithdrawCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status_by_card
                    .get_yearly_status_success_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearStatusSuccess {
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
                    year = req.year,
                    "find_yearly_withdraw_status_success_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_withdraw_status_success_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_withdraw_status_success_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraw_status_failed_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusWithdrawCardNumber {
                    card_number,
                    year: req.year,
                    month: req.month,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status_by_card
                    .get_month_status_failed_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthStatusFailed {
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
                    year = req.year,
                    month = req.month,
                    "find_monthly_withdraw_status_failed_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_withdraw_status_failed_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_withdraw_status_failed_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraw_status_failed_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearStatusWithdrawCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_status_by_card
                    .get_yearly_status_failed_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearStatusFailed {
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
                    year = req.year,
                    "find_yearly_withdraw_status_failed_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_withdraw_status_failed_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_withdraw_status_failed_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraws",
        year = request.get_ref().year
    ))]
    async fn find_monthly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
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
                    .withdraw_stats_amount
                    .get_monthly_withdraws(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_withdraws success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_withdraws rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_withdraws failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraws",
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
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
                    .withdraw_stats_amount
                    .get_yearly_withdraws(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_withdraws success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_withdraws rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_withdraws failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraws_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_monthly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearMonthCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_amount_by_card
                    .get_monthly_by_card_number(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawMonthAmount {
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
                    year = req.year,
                    "find_monthly_withdraws_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_withdraws_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_withdraws_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraws_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearMonthCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .withdraw_stats_amount_by_card
                    .get_yearly_by_card_number(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawYearAmount {
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
                    year = req.year,
                    "find_yearly_withdraws_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_withdraws_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_withdraws_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_card_number",
        card_number = tracing::field::Empty
    ))]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponsesWithdraw>, Status> {
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
                    .withdraw_query
                    .find_by_card(&card_number)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsesWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = masked_card, "find_by_card_number success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "find_by_card_number failed");
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
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
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
                    .withdraw_query
                    .find_by_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationWithdrawDeleteAt {
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
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
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
                    .withdraw_query
                    .find_by_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationWithdrawDeleteAt {
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
        method = "create_withdraw",
        card_number = tracing::field::Empty
    ))]
    async fn create_withdraw(
        &self,
        request: Request<CreateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let date = timestamp_to_naive_datetime(req.withdraw_time)
            .ok_or_else(|| Status::invalid_argument("withdraw_time invalid"))?;

        let domain_req = DomainCreateWithdrawRequest {
            card_number,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = masked_card, "create_withdraw success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "create_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "create_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_withdraw",
        withdraw_id = request.get_ref().withdraw_id,
        card_number = tracing::field::Empty
    ))]
    async fn update_withdraw(
        &self,
        request: Request<UpdateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let withdraw_id = req.withdraw_id;
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let date = timestamp_to_naive_datetime(req.withdraw_time)
            .ok_or_else(|| Status::invalid_argument("withdraw_time invalid"))?;

        let domain_req = DomainUpdateWithdrawRequest {
            card_number,
            withdraw_id: Some(req.withdraw_id),
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    withdraw_id = withdraw_id,
                    card_number = masked_card,
                    "update_withdraw success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            withdraw_id = withdraw_id,
                            "update_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(withdraw_id = withdraw_id, error = %inner, "update_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_withdraw", withdraw_id = request.get_ref().withdraw_id))]
    async fn trashed_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let withdraw_id = req.withdraw_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .trashed_withdraw(withdraw_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(withdraw_id = withdraw_id, "trashed_withdraw success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            withdraw_id = withdraw_id,
                            "trashed_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(withdraw_id = withdraw_id, error = %inner, "trashed_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_withdraw", withdraw_id = request.get_ref().withdraw_id))]
    async fn restore_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let withdraw_id = req.withdraw_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .restore(withdraw_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(withdraw_id = withdraw_id, "restore_withdraw success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            withdraw_id = withdraw_id,
                            "restore_withdraw rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(withdraw_id = withdraw_id, error = %inner, "restore_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_withdraw_permanent", withdraw_id = request.get_ref().withdraw_id))]
    async fn delete_withdraw_permanent(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let withdraw_id = req.withdraw_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .delete_permanent(withdraw_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    withdraw_id = withdraw_id,
                    "delete_withdraw_permanent success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            withdraw_id = withdraw_id,
                            "delete_withdraw_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(withdraw_id = withdraw_id, error = %inner, "delete_withdraw_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_withdraw"))]
    async fn restore_all_withdraw(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_withdraw success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_withdraw rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_withdraw failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_withdraw_permanent"))]
    async fn delete_all_withdraw_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .withdraw_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_withdraw_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_withdraw_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_withdraw_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
