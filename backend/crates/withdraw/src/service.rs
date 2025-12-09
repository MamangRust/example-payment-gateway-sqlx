use crate::di::DependenciesInject;
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
    errors::AppErrorGrpc,
    utils::{mask_card_number, timestamp_to_naive_datetime},
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct WithdrawServiceImpl {
    pub di: Arc<DependenciesInject>,
}

impl WithdrawServiceImpl {
    pub fn new(di: Arc<DependenciesInject>) -> Self {
        Self { di }
    }
}

#[tonic::async_trait]
impl WithdrawService for WithdrawServiceImpl {
    async fn find_all_withdraw(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_all_withdraw request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.withdraw_query.find_all(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Withdraw records fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to fetch withdraw records: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    async fn find_all_withdraw_by_card_number(
        &self,
        request: Request<FindAllWithdrawByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_all_withdraw_by_card_number request: card_number=****, page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllWithdrawCardNumber {
            card_number: req.card_number.clone(),
            search: req.search,
            page: req.page,
            page_size: req.page_size,
        };

        match self
            .di
            .withdraw_query
            .find_all_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Withdraw records by card fetched successfully: card_number=****, page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch withdraw records by card number: card_number=****, error={:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    async fn find_by_id_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_id_withdraw request: withdraw_id={}",
            req.withdraw_id
        );

        match self.di.withdraw_query.find_by_id(req.withdraw_id).await {
            Ok(api_response) => {
                info!(
                    "Withdraw record fetched successfully: withdraw_id={}",
                    req.withdraw_id
                );
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch withdraw record by ID {}: {:?}",
                    req.withdraw_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    async fn find_monthly_withdraw_status_success(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_monthly_withdraw_status_success request: year={}, month={}",
            req.year, req.month
        );

        let domain_req = MonthStatusWithdraw {
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .withdraw_stats_status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw status fetched successfully: year={}, month={}",
                    req.year, req.month
                );
                let grpc_response = ApiResponseWithdrawMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw status for {}/{}: {:?}",
                    req.year, req.month, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(
        skip(self, request),
        fields(method = "find_yearly_withdraw_status_success")
    )]
    async fn find_yearly_withdraw_status_success(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_yearly_withdraw_status_success request for year={}",
            req.year
        );

        match self
            .di
            .withdraw_stats_status
            .get_yearly_status_success(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw success status fetched successfully for year={}",
                    req.year
                );
                let grpc_response = ApiResponseWithdrawYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw success status for year={}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_monthly_withdraw_status_failed")
    )]
    async fn find_monthly_withdraw_status_failed(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_monthly_withdraw_status_failed request for year={}, month={}",
            req.year, req.month
        );

        let domain_req = MonthStatusWithdraw {
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .withdraw_stats_status
            .get_month_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw failed status fetched successfully for {}/{}",
                    req.year, req.month
                );
                let grpc_response = ApiResponseWithdrawMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw failed status for {}/{}: {:?}",
                    req.year, req.month, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_yearly_withdraw_status_failed")
    )]
    async fn find_yearly_withdraw_status_failed(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_yearly_withdraw_status_failed request for year={}",
            req.year
        );

        match self
            .di
            .withdraw_stats_status
            .get_yearly_status_failed(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw failed status fetched successfully for year={}",
                    req.year
                );
                let grpc_response = ApiResponseWithdrawYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw failed status for year={}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_monthly_withdraw_status_success_card_number")
    )]
    async fn find_monthly_withdraw_status_success_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);

        info!(
            "Received find_monthly_withdraw_status_success_card_number request for card={}, year={}, month={}",
            masked_card, req.year, req.month
        );

        let domain_req = MonthStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .withdraw_stats_status_by_card
            .get_month_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw success status by card fetched successfully for card={}, {}/{}",
                    masked_card, req.year, req.month
                );
                let grpc_response = ApiResponseWithdrawMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw success status by card for card={}, {}/{}: {:?}",
                    masked_card, req.year, req.month, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_yearly_withdraw_status_success_card_number")
    )]
    async fn find_yearly_withdraw_status_success_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);

        info!(
            "Received find_yearly_withdraw_status_success_card_number request for card={}, year={}",
            masked_card, req.year
        );

        let domain_req = YearStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .di
            .withdraw_stats_status_by_card
            .get_yearly_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw success status by card fetched successfully for card={}, year={}",
                    masked_card, req.year
                );
                let grpc_response = ApiResponseWithdrawYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw success status by card for card={}, year={}: {:?}",
                    masked_card, req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_monthly_withdraw_status_failed_card_number")
    )]
    async fn find_monthly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);

        info!(
            "Received find_monthly_withdraw_status_failed_card_number request for card={}, year={}, month={}",
            masked_card, req.year, req.month
        );

        let domain_req = MonthStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .withdraw_stats_status_by_card
            .get_month_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw failed status by card fetched successfully for card={}, {}/{}",
                    masked_card, req.year, req.month
                );
                let grpc_response = ApiResponseWithdrawMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw failed status by card for card={}, {}/{}: {:?}",
                    masked_card, req.year, req.month, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_yearly_withdraw_status_failed_card_number")
    )]
    async fn find_yearly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);

        info!(
            "Received find_yearly_withdraw_status_failed_card_number request for card={}, year={}",
            masked_card, req.year
        );

        let domain_req = YearStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .di
            .withdraw_stats_status_by_card
            .get_yearly_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw failed status by card fetched successfully for card={}, year={}",
                    masked_card, req.year
                );
                let grpc_response = ApiResponseWithdrawYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw failed status by card for card={}, year={}: {:?}",
                    masked_card, req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_monthly_withdraws"))]
    async fn find_monthly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_monthly_withdraws request for year={}",
            req.year
        );

        match self
            .di
            .withdraw_stats_amount
            .get_monthly_withdraws(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw amounts fetched successfully for year={}",
                    req.year
                );
                let grpc_response = ApiResponseWithdrawMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw amounts for year={}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_yearly_withdraws"))]
    async fn find_yearly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_yearly_withdraws request for year={}",
            req.year
        );

        match self
            .di
            .withdraw_stats_amount
            .get_yearly_withdraws(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw amounts fetched successfully for year={}",
                    req.year
                );
                let grpc_response = ApiResponseWithdrawYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw amounts for year={}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_monthly_withdraws_by_card_number")
    )]
    async fn find_monthly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "Received find_monthly_withdraws_by_card_number request for card={}, year={}",
            masked_card, req.year
        );

        let domain_req = YearMonthCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .di
            .withdraw_stats_amount_by_card
            .get_monthly_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Monthly withdraw amounts by card fetched successfully for card={}, year={}",
                    masked_card, req.year
                );
                let grpc_response = ApiResponseWithdrawMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch monthly withdraw amounts by card for card={}, year={}: {:?}",
                    masked_card, req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip(self, request),
        fields(method = "find_yearly_withdraws_by_card_number")
    )]
    async fn find_yearly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "Received find_yearly_withdraws_by_card_number request for card={}, year={}",
            masked_card, req.year
        );

        let domain_req = YearMonthCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .di
            .withdraw_stats_amount_by_card
            .get_yearly_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Yearly withdraw amounts by card fetched successfully for card={}, year={}",
                    masked_card, req.year
                );
                let grpc_response = ApiResponseWithdrawYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch yearly withdraw amounts by card for card={}, year={}: {:?}",
                    masked_card, req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_card_number"))]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponsesWithdraw>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "Received find_by_card_number request for card={}",
            masked_card
        );

        match self.di.withdraw_query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                info!(
                    "Withdraw records by card fetched successfully for card={}",
                    masked_card
                );
                let grpc_response = ApiResponsesWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch withdraw records by card for card={}: {:?}",
                    masked_card, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_active"))]
    async fn find_by_active(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_active request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.withdraw_query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Active withdraw records fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationWithdrawDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch active withdraw records: page={}, page_size={}, error={:?}",
                    req.page, req.page_size, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "find_by_trashed"))]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received find_by_trashed request: page={}, page_size={}, search={:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.withdraw_query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Trashed withdraw records fetched successfully: page={}, page_size={}",
                    domain_req.page, domain_req.page_size
                );
                let grpc_response = ApiResponsePaginationWithdrawDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to fetch trashed withdraw records: page={}, page_size={}, error={:?}",
                    req.page, req.page_size, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "create_withdraw"))]
    async fn create_withdraw(
        &self,
        request: Request<CreateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "Received create_withdraw request for card={}, amount={}, withdraw_time={:?}",
            masked_card, req.withdraw_amount, req.withdraw_time
        );

        let date = timestamp_to_naive_datetime(req.withdraw_time).ok_or_else(|| {
            let err_msg = "Invalid withdraw_time timestamp";
            error!("{} for card={}", err_msg, masked_card);
            Status::invalid_argument(err_msg)
        })?;

        let domain_req = DomainCreateWithdrawRequest {
            card_number: req.card_number,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        match self.di.withdraw_command.create(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Withdraw created successfully for card={}, amount={}",
                    masked_card, req.withdraw_amount
                );
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to create withdraw for card={}, amount={}: {:?}",
                    masked_card, req.withdraw_amount, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "update_withdraw"))]
    async fn update_withdraw(
        &self,
        request: Request<UpdateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "Received update_withdraw request for withdraw_id={}, card={}, amount={}, withdraw_time={:?}",
            req.withdraw_id, masked_card, req.withdraw_amount, req.withdraw_time
        );

        let date = timestamp_to_naive_datetime(req.withdraw_time).ok_or_else(|| {
            let err_msg = "Invalid withdraw_time timestamp";
            error!("{err_msg} for withdraw_id={}", req.withdraw_id);
            Status::invalid_argument(err_msg)
        })?;

        let domain_req = DomainUpdateWithdrawRequest {
            card_number: req.card_number,
            withdraw_id: Some(req.withdraw_id),
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        match self.di.withdraw_command.update(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "Withdraw updated successfully for withdraw_id={}, card={}",
                    req.withdraw_id, masked_card
                );
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to update withdraw for withdraw_id={}, card={}: {:?}",
                    req.withdraw_id, masked_card, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "trashed_withdraw"))]
    async fn trashed_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received trashed_withdraw request for withdraw_id={}",
            req.withdraw_id
        );

        match self
            .di
            .withdraw_command
            .trashed_withdraw(req.withdraw_id)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Withdraw trashed successfully for withdraw_id={}",
                    req.withdraw_id
                );
                let grpc_response = ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to trash withdraw for withdraw_id={}: {:?}",
                    req.withdraw_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), fields(method = "restore_withdraw"))]
    async fn restore_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "Received restore_withdraw request for withdraw_id={}",
            req.withdraw_id
        );

        match self.di.withdraw_command.restore(req.withdraw_id).await {
            Ok(api_response) => {
                info!(
                    "Withdraw restored successfully for withdraw_id={}",
                    req.withdraw_id
                );
                let grpc_response = ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to restore withdraw for withdraw_id={}: {:?}",
                    req.withdraw_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip(self, request), fields(method = "delete_withdraw_permanent"))]
    async fn delete_withdraw_permanent(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDelete>, Status> {
        let req = request.into_inner();
        info!(
            "Received delete_withdraw_permanent request for withdraw_id={}",
            req.withdraw_id
        );

        match self
            .di
            .withdraw_command
            .delete_permanent(req.withdraw_id)
            .await
        {
            Ok(api_response) => {
                info!(
                    "Withdraw permanently deleted for withdraw_id={}",
                    req.withdraw_id
                );
                let grpc_response = ApiResponseWithdrawDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "Failed to permanently delete withdraw for withdraw_id={}: {:?}",
                    req.withdraw_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "restore_all_withdraw"))]
    async fn restore_all_withdraw(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        info!("Received restore_all_withdraw request");

        match self.di.withdraw_command.restore_all().await {
            Ok(api_response) => {
                info!("All trashed withdraws restored successfully");
                let grpc_response = ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to restore all withdraws: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), fields(method = "delete_all_withdraw_permanent"))]
    async fn delete_all_withdraw_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        info!("Received delete_all_withdraw_permanent request");

        match self.di.withdraw_command.delete_all().await {
            Ok(api_response) => {
                info!("All withdraws permanently deleted successfully");
                let grpc_response = ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("Failed to permanently delete all withdraws: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
