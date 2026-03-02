use genproto::transfer::{
    ApiResponsePaginationTransfer, ApiResponsePaginationTransferDeleteAt, ApiResponseTransfer,
    ApiResponseTransferAll, ApiResponseTransferDelete, ApiResponseTransferDeleteAt,
    ApiResponseTransferMonthAmount, ApiResponseTransferMonthStatusFailed,
    ApiResponseTransferMonthStatusSuccess, ApiResponseTransferYearAmount,
    ApiResponseTransferYearStatusFailed, ApiResponseTransferYearStatusSuccess,
    ApiResponseTransfers, CreateTransferRequest, FindAllTransferRequest,
    FindByCardNumberTransferRequest, FindByIdTransferRequest, FindMonthlyTransferStatus,
    FindMonthlyTransferStatusCardNumber, FindTransferByTransferFromRequest,
    FindTransferByTransferToRequest, FindYearTransferStatus, FindYearTransferStatusCardNumber,
    UpdateTransferRequest, transfer_service_server::TransferService,
};
use shared::{
    domain::requests::transfer::{
        CreateTransferRequest as DomainCreateTransferRequest, FindAllTransfers,
        MonthStatusTransfer, MonthStatusTransferCardNumber, MonthYearCardNumber,
        UpdateTransferRequest as DomainUpdateTransferRequest, YearStatusTransferCardNumber,
    },
    errors::{AppErrorGrpc, CircuitBreakerError},
    utils::mask_card_number,
};
use std::sync::Arc;
use tracing::{error, info, instrument, warn};

use crate::state::AppState;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct TransferServiceImpl {
    pub state: Arc<AppState>,
}

impl TransferServiceImpl {
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
impl TransferService for TransferServiceImpl {
    #[instrument(skip(self, request), fields(
        method = "find_all_transfer",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransfer>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransfers {
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
                    .transfer_query
                    .find_all(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransfer {
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
                    "find_all_transfer success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_all_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_all_transfer failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_id_transfer", transfer_id = request.get_ref().transfer_id))]
    async fn find_by_id_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_id = req.transfer_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_query
                    .find_by_id(transfer_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(transfer_id = transfer_id, "find_by_id_transfer success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_id = transfer_id,
                            "find_by_id_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_id = transfer_id, error = %inner, "find_by_id_transfer failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_status_success",
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_transfer_status_success(
        &self,
        request: Request<FindMonthlyTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransfer { year, month };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status
                    .get_month_status_success(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthStatusSuccess {
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
                    "find_monthly_transfer_status_success success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_transfer_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_transfer_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_status_success",
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_status_success(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearStatusSuccess>, Status> {
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
                    .transfer_stats_status
                    .get_yearly_status_success(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transfer_status_success success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transfer_status_success rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_transfer_status_success failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_status_failed",
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_transfer_status_failed(
        &self,
        request: Request<FindMonthlyTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let year = req.year;
        let month = req.month;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransfer { year, month };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status
                    .get_month_status_failed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthStatusFailed {
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
                    "find_monthly_transfer_status_failed success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            month = month,
                            "find_monthly_transfer_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, month = month, error = %inner, "find_monthly_transfer_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_status_failed",
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_status_failed(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearStatusFailed>, Status> {
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
                    .transfer_stats_status
                    .get_yearly_status_failed(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transfer_status_failed success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transfer_status_failed rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_transfer_status_failed failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_transfer_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferMonthStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransferCardNumber {
                    card_number,
                    year: req.year,
                    month: req.month,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status_by_card
                    .get_month_status_success_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthStatusSuccess {
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
                    "find_monthly_transfer_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transfer_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transfer_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_status_success_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_status_success_by_card_number(
        &self,
        request: Request<FindYearTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferYearStatusSuccess>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearStatusTransferCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status_by_card
                    .get_yearly_status_success_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearStatusSuccess {
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
                    "find_yearly_transfer_status_success_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transfer_status_success_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transfer_status_success_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year,
        month = request.get_ref().month
    ))]
    async fn find_monthly_transfer_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferMonthStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthStatusTransferCardNumber {
                    card_number,
                    year: req.year,
                    month: req.month,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status_by_card
                    .get_month_status_failed_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthStatusFailed {
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
                    "find_monthly_transfer_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transfer_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transfer_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_status_failed_by_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_status_failed_by_card_number(
        &self,
        request: Request<FindYearTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferYearStatusFailed>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = YearStatusTransferCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_status_by_card
                    .get_yearly_status_failed_by_card(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearStatusFailed {
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
                    "find_yearly_transfer_status_failed_by_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transfer_status_failed_by_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transfer_status_failed_by_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_amounts",
        year = request.get_ref().year
    ))]
    async fn find_monthly_transfer_amounts(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
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
                    .transfer_stats_amount
                    .get_monthly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_monthly_transfer_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_monthly_transfer_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_monthly_transfer_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_amounts",
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_amounts(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
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
                    .transfer_stats_amount
                    .get_yearly_amounts(year)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(year = year, "find_yearly_transfer_amounts success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            year = year,
                            "find_yearly_transfer_amounts rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(year = year, error = %inner, "find_yearly_transfer_amounts failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_amounts_by_sender_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_monthly_transfer_amounts_by_sender_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthYearCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_amount_by_card
                    .get_monthly_amounts_by_sender(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthAmount {
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
                    "find_monthly_transfer_amounts_by_sender_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transfer_amounts_by_sender_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transfer_amounts_by_sender_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_amounts_by_sender_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_amounts_by_sender_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthYearCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_amount_by_card
                    .get_yearly_amounts_by_sender(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearAmount {
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
                    "find_yearly_transfer_amounts_by_sender_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transfer_amounts_by_sender_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transfer_amounts_by_sender_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_monthly_transfer_amounts_by_receiver_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_monthly_transfer_amounts_by_receiver_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthYearCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_amount_by_card
                    .get_monthly_amounts_by_receiver(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferMonthAmount {
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
                    "find_monthly_transfer_amounts_by_receiver_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_monthly_transfer_amounts_by_receiver_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_monthly_transfer_amounts_by_receiver_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_yearly_transfer_amounts_by_receiver_card_number",
        card_number = tracing::field::Empty,
        year = request.get_ref().year
    ))]
    async fn find_yearly_transfer_amounts_by_receiver_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let card_number = req.card_number.clone();
        let masked_card = mask_card_number(&card_number);
        tracing::Span::current().record("card_number", &masked_card);

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let domain_req = MonthYearCardNumber {
                    card_number,
                    year: req.year,
                };

                let api_response = self
                    .state
                    .di_container
                    .transfer_stats_amount_by_card
                    .get_yearly_amounts_by_receiver(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferYearAmount {
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
                    "find_yearly_transfer_amounts_by_receiver_card_number success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            card_number = masked_card,
                            "find_yearly_transfer_amounts_by_receiver_card_number rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            card_number = masked_card,
                            error = %inner,
                            "find_yearly_transfer_amounts_by_receiver_card_number failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_transfer_by_transfer_from",
        transfer_from = %request.get_ref().transfer_from
    ))]
    async fn find_transfer_by_transfer_from(
        &self,
        request: Request<FindTransferByTransferFromRequest>,
    ) -> Result<Response<ApiResponseTransfers>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_from = req.transfer_from.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_query
                    .find_by_transfer_from(&transfer_from)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransfers {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transfer_from = transfer_from,
                    "find_transfer_by_transfer_from success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_from = transfer_from,
                            "find_transfer_by_transfer_from rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_from = transfer_from, error = %inner, "find_transfer_by_transfer_from failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_transfer_by_transfer_to",
        transfer_to = %request.get_ref().transfer_to
    ))]
    async fn find_transfer_by_transfer_to(
        &self,
        request: Request<FindTransferByTransferToRequest>,
    ) -> Result<Response<ApiResponseTransfers>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_to = req.transfer_to.clone();

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_query
                    .find_by_transfer_to(&transfer_to)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransfers {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transfer_to = transfer_to,
                    "find_transfer_by_transfer_to success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_to = transfer_to,
                            "find_transfer_by_transfer_to rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_to = transfer_to, error = %inner, "find_by_transfer_to failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_active_transfer",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_active_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransferDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransfers {
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
                    .transfer_query
                    .find_by_active(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransferDeleteAt {
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
                    "find_by_active_transfer success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_active_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_active_transfer failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "find_by_trashed_transfer",
        page = request.get_ref().page,
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_trashed_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransferDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let domain_req = FindAllTransfers {
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
                    .transfer_query
                    .find_by_trashed(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponsePaginationTransferDeleteAt {
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
                    "find_by_trashed_transfer success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            "find_by_trashed_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            page = domain_req.page,
                            page_size = domain_req.page_size,
                            error = %inner,
                            "find_by_trashed_transfer failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "create_transfer",
        transfer_from = %request.get_ref().transfer_from,
        transfer_to = %request.get_ref().transfer_to
    ))]
    async fn create_transfer(
        &self,
        request: Request<CreateTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_from = req.transfer_from.clone();
        let transfer_to = req.transfer_to.clone();

        let domain_req = DomainCreateTransferRequest {
            transfer_from: transfer_from.clone(),
            transfer_to: transfer_to.clone(),
            transfer_amount: req.transfer_amount as i64,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .create(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transfer_from = transfer_from,
                    transfer_to = transfer_to,
                    "create_transfer success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_from = transfer_from,
                            "create_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(
                            transfer_from = transfer_from,
                            error = %inner,
                            "create_transfer failed"
                        );
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(
        method = "update_transfer",
        transfer_id = request.get_ref().transfer_id
    ))]
    async fn update_transfer(
        &self,
        request: Request<UpdateTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_id = req.transfer_id;

        let domain_req = DomainUpdateTransferRequest {
            transfer_id: Some(req.transfer_id),
            transfer_from: req.transfer_from,
            transfer_to: req.transfer_to,
            transfer_amount: req.transfer_amount as i64,
        };

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .update(&domain_req)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(transfer_id = transfer_id, "update_transfer success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_id = transfer_id,
                            "update_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_id = transfer_id, error = %inner, "update_transfer failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_transfer", transfer_id = request.get_ref().transfer_id))]
    async fn trashed_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_id = req.transfer_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .trashed(transfer_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(transfer_id = transfer_id, "trashed_transfer success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_id = transfer_id,
                            "trashed_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_id = transfer_id, error = %inner, "trashed_transfer failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_transfer", transfer_id = request.get_ref().transfer_id))]
    async fn restore_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDeleteAt>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_id = req.transfer_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .restore(transfer_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(transfer_id = transfer_id, "restore_transfer success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_id = transfer_id,
                            "restore_transfer rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_id = transfer_id, error = %inner, "restore_transfer failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "delete_transfer_permanent", transfer_id = request.get_ref().transfer_id))]
    async fn delete_transfer_permanent(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDelete>, Status> {
        self.check_rate_limit().await?;

        let req = request.into_inner();
        let transfer_id = req.transfer_id;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .delete_permanent(transfer_id)
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!(
                    transfer_id = transfer_id,
                    "delete_transfer_permanent success"
                );
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!(
                            transfer_id = transfer_id,
                            "delete_transfer_permanent rejected: circuit breaker open"
                        );
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(transfer_id = transfer_id, error = %inner, "delete_transfer_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_transfer"))]
    async fn restore_all_transfer(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransferAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .restore_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("restore_all_transfer success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("restore_all_transfer rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "restore_all_transfer failed");
                    }
                }
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_transfer_permanent"))]
    async fn delete_all_transfer_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransferAll>, Status> {
        self.check_rate_limit().await?;

        let result = self
            .state
            .circuit_breaker
            .call_async(|| async {
                let api_response = self
                    .state
                    .di_container
                    .transfer_command
                    .delete_all()
                    .await
                    .map_err(AppErrorGrpc::from)?;

                Ok(Response::new(ApiResponseTransferAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            })
            .await;

        match result {
            Ok(resp) => {
                info!("delete_all_transfer_permanent success");
                Ok(resp)
            }
            Err(e) => {
                match &e {
                    CircuitBreakerError::Open => {
                        warn!("delete_all_transfer_permanent rejected: circuit breaker open");
                    }
                    CircuitBreakerError::Inner(inner) => {
                        error!(error = %inner, "delete_all_transfer_permanent failed");
                    }
                }
                Err(e.into())
            }
        }
    }
}
