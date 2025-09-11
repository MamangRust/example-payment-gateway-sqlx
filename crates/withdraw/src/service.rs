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
    abstract_trait::withdraw::service::{
        command::DynWithdrawCommandService,
        query::DynWithdrawQueryService,
        stats::{amount::DynWithdrawStatsAmountService, status::DynWithdrawStatsStatusService},
        statsbycard::{
            amount::DynWithdrawStatsAmountByCardService,
            status::DynWithdrawStatsStatusByCardService,
        },
    },
    domain::requests::withdraw::{
        CreateWithdrawRequest as DomainCreateWithdrawRequest, FindAllWithdrawCardNumber,
        FindAllWithdraws, MonthStatusWithdraw, MonthStatusWithdrawCardNumber,
        UpdateWithdrawRequest as DomainUpdateWithdrawRequest, YearMonthCardNumber,
        YearStatusWithdrawCardNumber,
    },
    errors::AppErrorGrpc,
    utils::timestamp_to_naive_datetime,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct WithdrawStats {
    pub amount: DynWithdrawStatsAmountService,
    pub status: DynWithdrawStatsStatusService,
}

#[derive(Clone)]
pub struct WithdrawStatsByCard {
    pub amount: DynWithdrawStatsAmountByCardService,
    pub status: DynWithdrawStatsStatusByCardService,
}

#[derive(Clone)]
pub struct WithdrawServiceImpl {
    pub query: DynWithdrawQueryService,
    pub command: DynWithdrawCommandService,
    pub stats: WithdrawStats,
    pub statsbycard: WithdrawStatsByCard,
}

impl WithdrawServiceImpl {
    pub fn new(
        query: DynWithdrawQueryService,
        command: DynWithdrawCommandService,
        stats: WithdrawStats,
        statsbycard: WithdrawStatsByCard,
    ) -> Self {
        Self {
            query,
            command,
            stats,
            statsbycard,
        }
    }
}

#[tonic::async_trait]
impl WithdrawService for WithdrawServiceImpl {
    async fn find_all_withdraw(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_all_withdraw_by_card_number(
        &self,
        request: Request<FindAllWithdrawByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdraw>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllWithdrawCardNumber {
            card_number: req.card_number,
            search: req.search,
            page: req.page,
            page_size: req.page_size,
        };

        match self.query.find_all_by_card_number(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_id_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();

        match self.query.find_by_id(req.withdraw_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraw_status_success(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusWithdraw {
            year: req.year,
            month: req.month,
        };

        match self
            .stats
            .status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraw_status_success(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_success(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraw_status_failed(
        &self,
        request: Request<FindMonthlyWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusWithdraw {
            year: req.year,
            month: req.month,
        };

        match self.stats.status.get_month_status_failed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraw_status_failed(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_failed(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraw_status_success_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraw_status_success_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindMonthlyWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraw_status_failed_card_number(
        &self,
        request: Request<FindYearWithdrawStatusCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusWithdrawCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_monthly_withdraws(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraws(
        &self,
        request: Request<FindYearWithdrawStatus>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_yearly_withdraws(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawMonthAmount>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_monthly_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_withdraws_by_card_number(
        &self,
        request: Request<FindYearWithdrawCardNumber>,
    ) -> Result<Response<ApiResponseWithdrawYearAmount>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_yearly_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponsesWithdraw>, Status> {
        let req = request.into_inner();

        match self.query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsesWithdraw {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_active(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationWithdrawDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_trashed(
        &self,
        request: Request<FindAllWithdrawRequest>,
    ) -> Result<Response<ApiResponsePaginationWithdrawDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllWithdraws {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationWithdrawDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn create_withdraw(
        &self,
        request: Request<CreateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_datetime(req.withdraw_time)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainCreateWithdrawRequest {
            card_number: req.card_number,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_withdraw(
        &self,
        request: Request<UpdateWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdraw>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_datetime(req.withdraw_time)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainUpdateWithdrawRequest {
            card_number: req.card_number,
            withdraw_id: req.withdraw_id,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: date,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdraw {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trashed_withdraw(req.withdraw_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_withdraw(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.withdraw_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_withdraw_permanent(
        &self,
        request: Request<FindByIdWithdrawRequest>,
    ) -> Result<Response<ApiResponseWithdrawDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete_permanent(req.withdraw_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_withdraw(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_withdraw_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseWithdrawAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseWithdrawAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
