use crate::state::AppState;
use genproto::card::{
    ApiResponseCard, ApiResponseCardAll, ApiResponseCardDelete, ApiResponseCardDeleteAt,
    ApiResponseDashboardCard, ApiResponseDashboardCardNumber, ApiResponseMonthlyAmount,
    ApiResponseMonthlyBalance, ApiResponsePaginationCard, ApiResponsePaginationCardDeleteAt,
    ApiResponseYearlyAmount, ApiResponseYearlyBalance, CreateCardRequest, FindAllCardRequest,
    FindByCardNumberRequest, FindByIdCardRequest, FindByUserIdCardRequest, FindYearAmount,
    FindYearAmountCardNumber, FindYearBalance, FindYearBalanceCardNumber, UpdateCardRequest,
    card_service_server::CardService,
};
use shared::{
    domain::requests::card::{
        CreateCardRequest as DomainCreateCardRequest, FindAllCards, MonthYearCardNumberCard,
        UpdateCardRequest as DomainUpdateCardRequest,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::timestamp_to_naive_date,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
pub struct CardServiceImpl {
    pub state: Arc<AppState>,
}

impl CardServiceImpl {
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
impl CardService for CardServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_card",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCard>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllCards {
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
                    .card_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let data: Vec<genproto::card::CardResponse> =
                    api_response.data.into_iter().map(Into::into).collect();

                Ok(Response::new(ApiResponsePaginationCard {
                    status: api_response.status,
                    message: api_response.message,
                    data,
                    pagination: Some(api_response.pagination.into()),
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    page = domain_req.page,
                    page_size = domain_req.page_size,
                    "find_all_card success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_card failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_card", card_id = request.get_ref().card_id))]
    async fn find_by_id_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_id = req.card_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_query
                    .find_by_id(card_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_id = card_id, "find_by_id_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_id = card_id,
                            "find_by_id_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_id = card_id, error = %inner, "find_by_id_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_user_id_card", user_id = request.get_ref().user_id))]
    async fn find_by_user_id_card(
        &self,
        request: Request<FindByUserIdCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
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
                    .card_query
                    .find_by_user_id(user_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "find_by_user_id_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "find_by_user_id_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "find_by_user_id_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_active_card",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_active_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCardDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllCards {
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
                    .card_query
                    .find_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let data: Vec<genproto::card::CardResponseDeleteAt> =
                    api_response.data.into_iter().map(Into::into).collect();

                Ok(Response::new(ApiResponsePaginationCardDeleteAt {
                    data,
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
                    "find_by_active_card success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active_card failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_trashed_card",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_trashed_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCardDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllCards {
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
                    .card_query
                    .find_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                let data: Vec<genproto::card::CardResponseDeleteAt> =
                    api_response.data.into_iter().map(Into::into).collect();

                Ok(Response::new(ApiResponsePaginationCardDeleteAt {
                    data,
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
                    "find_by_trashed_card success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed_card failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_card_number", card_number = request.get_ref().card_number))]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_query
                    .find_by_card(&card_number)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = card_number, "find_by_card_number success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = card_number,
                            "find_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_number = card_number, error = %inner, "find_by_card_number failed");
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, _request), fields(method = "dashboard_card"))]
    async fn dashboard_card(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseDashboardCard>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_dashboard
                    .get_dashboard()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseDashboardCard {
                    message: api_response.message,
                    status: api_response.status,
                    data: Some(api_response.data.into()),
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("dashboard_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("dashboard_card rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "dashboard_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "dashboard_card_number", card_number = request.get_ref().card_number))]
    async fn dashboard_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseDashboardCardNumber>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_dashboard
                    .get_dashboard_bycard(card_number.clone())
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseDashboardCardNumber {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_number = card_number, "dashboard_card_number success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = card_number,
                            "dashboard_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = card_number,
                            error = %inner,
                            "dashboard_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_balance", year = request.get_ref().year))]
    async fn find_monthly_balance(
        &self,
        request: Request<FindYearBalance>,
    ) -> Result<Response<ApiResponseMonthlyBalance>, Status> {
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
                    .stats_balance
                    .get_monthly_balance(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_balance success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_balance rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_balance failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_balance", year = request.get_ref().year))]
    async fn find_yearly_balance(
        &self,
        request: Request<FindYearBalance>,
    ) -> Result<Response<ApiResponseYearlyBalance>, Status> {
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
                    .stats_balance
                    .get_yearly_balance(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_balance success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_balance rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_balance failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(method = "find_monthly_topup_amount", year = request.get_ref().year))]
    async fn find_monthly_topup_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
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
                    .stats_topup
                    .get_monthly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_topup_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_topup_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_topup_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_topup_amount", year = request.get_ref().year))]
    async fn find_yearly_topup_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
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
                    .stats_topup
                    .get_yearly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_topup_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_topup_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_topup_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_withdraw_amount", year = request.get_ref().year))]
    async fn find_monthly_withdraw_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
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
                    .stats_withdraw
                    .get_monthly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_withdraw_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_withdraw_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_withdraw_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_withdraw_amount", year = request.get_ref().year))]
    async fn find_yearly_withdraw_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
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
                    .stats_withdraw
                    .get_yearly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_withdraw_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_withdraw_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_withdraw_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(method = "find_monthly_transaction_amount", year = request.get_ref().year))]
    async fn find_monthly_transaction_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
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
                    .stats_transaction
                    .get_monthly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_transaction_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_transaction_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_transaction_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_transaction_amount", year = request.get_ref().year))]
    async fn find_yearly_transaction_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
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
                    .stats_transaction
                    .get_yearly_amount(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transaction_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transaction_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_transaction_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_transfer_sender_amount", year = request.get_ref().year))]
    async fn find_monthly_transfer_sender_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
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
                    .stats_transfer
                    .get_monthly_amount_sender(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_transfer_sender_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_transfer_sender_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_transfer_sender_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_transfer_sender_amount", year = request.get_ref().year))]
    async fn find_yearly_transfer_sender_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
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
                    .stats_transfer
                    .get_yearly_amount_sender(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transfer_sender_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transfer_sender_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_transfer_sender_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_transfer_receiver_amount", year = request.get_ref().year))]
    async fn find_monthly_transfer_receiver_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
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
                    .stats_transfer
                    .get_monthly_amount_receiver(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_transfer_receiver_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_transfer_receiver_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_monthly_transfer_receiver_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_transfer_receiver_amount", year = request.get_ref().year))]
    async fn find_yearly_transfer_receiver_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
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
                    .stats_transfer
                    .get_yearly_amount_receiver(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transfer_receiver_amount success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transfer_receiver_amount rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            year = year,
                            error = %inner,
                            "find_yearly_transfer_receiver_amount failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(
        method = "find_monthly_balance_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_monthly_balance_by_card_number(
        &self,
        request: Request<FindYearBalanceCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyBalance>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_balance
                    .get_monthly_balance(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_balance_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_balance_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_balance_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_balance_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_yearly_balance_by_card_number(
        &self,
        request: Request<FindYearBalanceCardNumber>,
    ) -> Result<Response<ApiResponseYearlyBalance>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_balance
                    .get_yearly_balance(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_balance_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_balance_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_balance_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_topup_amount_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_monthly_topup_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_topup
                    .get_monthly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_topup_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_topup_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_topup_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_topup_amount_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_yearly_topup_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_topup
                    .get_yearly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_topup_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_topup_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_topup_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_withdraw_amount_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_monthly_withdraw_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_withdraw
                    .get_monthly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_withdraw_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_withdraw_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_withdraw_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_withdraw_amount_by_card_number",
        year = request.get_ref().year,
        card_number = %request.get_ref().card_number
    ))]
    async fn find_yearly_withdraw_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_withdraw
                    .get_yearly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_withdraw_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_withdraw_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_withdraw_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }
    #[instrument(skip(self, request), fields(
        method = "find_monthly_transaction_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_monthly_transaction_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transaction
                    .get_monthly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_transaction_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_transaction_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_transaction_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transaction_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transaction_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transaction
                    .get_yearly_amount(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_transaction_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_transaction_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_transaction_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_sender_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_monthly_transfer_sender_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transfer
                    .get_monthly_amount_sender(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_transfer_sender_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_transfer_sender_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_transfer_sender_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_sender_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_sender_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transfer
                    .get_yearly_amount_sender(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_transfer_sender_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_transfer_sender_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_transfer_sender_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_receiver_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_monthly_transfer_receiver_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transfer
                    .get_monthly_amount_receiver(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_monthly_transfer_receiver_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_monthly_transfer_receiver_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_monthly_transfer_receiver_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_receiver_amount_by_card_number",
        card_number = %request.get_ref().card_number,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_receiver_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .stats_bycard_transfer
                    .get_yearly_amount_receiver(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    card_number = domain_req.card_number,
                    year = domain_req.year,
                    "find_yearly_transfer_receiver_amount_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            "find_yearly_transfer_receiver_amount_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = domain_req.card_number,
                            year = domain_req.year,
                            error = %inner,
                            "find_yearly_transfer_receiver_amount_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "create_card",
        user_id = request.get_ref().user_id,
        card_type = ?request.get_ref().card_type
    ))]
    async fn create_card(
        &self,
        request: Request<CreateCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let user_id = req.user_id;

        let date = timestamp_to_naive_date(req.expire_date)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainCreateCardRequest {
            user_id: req.user_id,
            card_type: req.card_type,
            expire_date: date,
            cvv: req.cvv,
            card_provider: req.card_provider,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(user_id = user_id, "create_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            user_id = user_id,
                            "create_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(user_id = user_id, error = %inner, "create_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_card",
        card_id = request.get_ref().card_id,
        user_id = request.get_ref().user_id
    ))]
    async fn update_card(
        &self,
        request: Request<UpdateCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_id = req.card_id;

        let date = timestamp_to_naive_date(req.expire_date)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainUpdateCardRequest {
            card_id: Some(req.card_id),
            user_id: req.user_id,
            card_type: req.card_type,
            expire_date: date,
            cvv: req.cvv,
            card_provider: req.card_provider,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_id = card_id, "update_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_id = card_id,
                            "update_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_id = card_id, error = %inner, "update_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_card", card_id = request.get_ref().card_id))]
    async fn trashed_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_id = req.card_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .trash(card_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCardDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_id = card_id, "trashed_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_id = card_id,
                            "trashed_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_id = card_id, error = %inner, "trashed_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_card", card_id = request.get_ref().card_id))]
    async fn restore_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_id = req.card_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .restore(card_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCardDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_id = card_id, "restore_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_id = card_id,
                            "restore_card rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_id = card_id, error = %inner, "restore_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_card_permanent", card_id = request.get_ref().card_id))]
    async fn delete_card_permanent(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_id = req.card_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .delete(card_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCardDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(card_id = card_id, "delete_card_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_id = card_id,
                            "delete_card_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(card_id = card_id, error = %inner, "delete_card_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_card"))]
    async fn restore_all_card(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseCardAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCardAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_card success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_card rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_card failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_card_permanent"))]
    async fn delete_all_card_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseCardAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .card_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseCardAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_card_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_card_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_card_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
