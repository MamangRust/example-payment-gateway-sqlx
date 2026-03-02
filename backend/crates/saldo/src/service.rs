use crate::state::AppState;
use genproto::{
    card::FindByCardNumberRequest,
    saldo::{
        ApiResponseMonthSaldoBalances, ApiResponseMonthTotalSaldo, ApiResponsePaginationSaldo,
        ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseSaldoAll,
        ApiResponseSaldoDelete, ApiResponseSaldoDeleteAt, ApiResponseYearSaldoBalances,
        ApiResponseYearTotalSaldo, CreateSaldoRequest, FindAllSaldoRequest, FindByIdSaldoRequest,
        FindMonthlySaldoTotalBalance, FindYearlySaldo, UpdateSaldoRequest,
        saldo_service_server::SaldoService,
    },
};
use shared::{
    domain::requests::saldo::{
        CreateSaldoRequest as DomainCreateSaldoRequest, FindAllSaldos, MonthTotalSaldoBalance,
        UpdateSaldoRequest as DomainUpdateSaldoRequest,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::mask_card_number,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct SaldoServiceImpl {
    pub state: Arc<AppState>,
}

impl SaldoServiceImpl {
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
impl SaldoService for SaldoServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_saldo",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ), level = "info")]
    async fn find_all_saldo(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldo>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllSaldos {
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
                    .saldo_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationSaldo {
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
                    "find_all_saldo success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_saldo failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_saldo", saldo_id = request.get_ref().saldo_id), level = "info")]
    async fn find_by_id_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let saldo_id = req.saldo_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_query
                    .find_by_id(saldo_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(saldo_id = saldo_id, "find_by_id_saldo success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            saldo_id = saldo_id,
                            "find_by_id_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(saldo_id = saldo_id, error = %inner, "find_by_id_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_total_saldo_balance",
        year = request.get_ref().year,
        month = request.get_ref().month
    ), level = "info")]
    async fn find_monthly_total_saldo_balance(
        &self,
        request: Request<FindMonthlySaldoTotalBalance>,
    ) -> Result<Response<ApiResponseMonthTotalSaldo>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthTotalSaldoBalance {
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
                    .saldo_total_balance
                    .get_month_total_balance(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthTotalSaldo {
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
                    "find_monthly_total_saldo_balance success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = domain_req.year,
                            month = domain_req.month,
                            "find_monthly_total_saldo_balance rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = domain_req.year,
                            month = domain_req.month,
                            error = %inner,
                            "find_monthly_total_saldo_balance failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_year_total_saldo_balance", year = request.get_ref().year), level = "info")]
    async fn find_year_total_saldo_balance(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearTotalSaldo>, Status> {
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
                    .saldo_total_balance
                    .get_year_total_balance(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearTotalSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_year_total_saldo_balance success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_year_total_saldo_balance rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_year_total_saldo_balance failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_saldo_balances", year = request.get_ref().year), level = "info")]
    async fn find_monthly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseMonthSaldoBalances>, Status> {
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
                    .saldo_balance
                    .get_month_balance(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthSaldoBalances {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_saldo_balances success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_saldo_balances rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_saldo_balances failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_saldo_balances", year = request.get_ref().year), level = "info")]
    async fn find_yearly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearSaldoBalances>, Status> {
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
                    .saldo_balance
                    .get_year_balance(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearSaldoBalances {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_saldo_balances success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_saldo_balances rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_saldo_balances failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_card_number", card_number = tracing::field::Empty), level = "info")]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
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
                    .saldo_query
                    .find_by_card(&card_number)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldo {
                    data: Some(api_response.data.into()),
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
    ), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllSaldos {
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
                    .saldo_query
                    .find_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationSaldoDeleteAt {
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
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllSaldos {
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
                    .saldo_query
                    .find_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationSaldoDeleteAt {
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
        method = "create_saldo",
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn create_saldo(
        &self,
        request: Request<CreateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = DomainCreateSaldoRequest {
            card_number,
            total_balance: req.total_balance as i64,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = masked_card, "create_saldo success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "create_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = masked_card, error = %inner, "create_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_saldo",
        saldo_id = request.get_ref().saldo_id,
        card_number = tracing::field::Empty
    ), level = "info")]
    async fn update_saldo(
        &self,
        request: Request<UpdateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let saldo_id = req.saldo_id;
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let domain_req = DomainUpdateSaldoRequest {
            saldo_id: Some(req.saldo_id),
            card_number,
            total_balance: req.total_balance as i64,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    saldo_id = saldo_id,
                    card_number = masked_card,
                    "update_saldo success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            saldo_id = saldo_id,
                            card_number = masked_card,
                            "update_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(saldo_id = saldo_id, card_number = masked_card, error = %inner, "update_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_saldo", saldo_id = request.get_ref().saldo_id), level = "info")]
    async fn trashed_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let saldo_id = req.saldo_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .trash(saldo_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(saldo_id = saldo_id, "trashed_saldo success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            saldo_id = saldo_id,
                            "trashed_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(saldo_id = saldo_id, error = %inner, "trashed_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_saldo", saldo_id = request.get_ref().saldo_id), level = "info")]
    async fn restore_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let saldo_id = req.saldo_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .restore(saldo_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(saldo_id = saldo_id, "restore_saldo success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            saldo_id = saldo_id,
                            "restore_saldo rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(saldo_id = saldo_id, error = %inner, "restore_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_saldo_permanent", saldo_id = request.get_ref().saldo_id), level = "info")]
    async fn delete_saldo_permanent(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let saldo_id = req.saldo_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .delete(saldo_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldoDelete {
                    status: api_response.status,
                    message: api_response.message,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(saldo_id = saldo_id, "delete_saldo_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            saldo_id = saldo_id,
                            "delete_saldo_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(saldo_id = saldo_id, error = %inner, "delete_saldo_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "restore_all_saldo"),
        level = "info"
    )]
    async fn restore_all_saldo(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_saldo success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_saldo rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_saldo failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(
        skip(self, _request),
        fields(method = "delete_all_saldo_permanent"),
        level = "info"
    )]
    async fn delete_all_saldo_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .saldo_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_saldo_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_saldo_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_saldo_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
