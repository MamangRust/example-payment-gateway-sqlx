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
    abstract_trait::topup::service::{
        command::DynTopupCommandService,
        query::DynTopupQueryService,
        stats::{
            amount::DynTopupStatsAmountService, method::DynTopupStatsMethodService,
            status::DynTopupStatsStatusService,
        },
        statsbycard::{
            amount::DynTopupStatsAmountByCardService, method::DynTopupStatsMethodByCardService,
            status::DynTopupStatsStatusByCardService,
        },
    },
    domain::requests::topup::{
        CreateTopupRequest as DomainCreateTopupRequest, FindAllTopups, FindAllTopupsByCardNumber,
        MonthTopupStatus, MonthTopupStatusCardNumber,
        UpdateTopupRequest as DomainUpdateTopupRequst, YearMonthMethod, YearTopupStatusCardNumber,
    },
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct TopupStats {
    pub amount: DynTopupStatsAmountService,
    pub method: DynTopupStatsMethodService,
    pub status: DynTopupStatsStatusService,
}

#[derive(Clone)]
pub struct TopupStatsByCard {
    pub amount: DynTopupStatsAmountByCardService,
    pub method: DynTopupStatsMethodByCardService,
    pub status: DynTopupStatsStatusByCardService,
}

#[derive(Clone)]
pub struct TopupServiceImpl {
    pub query: DynTopupQueryService,
    pub command: DynTopupCommandService,
    pub stats: TopupStats,
    pub statsbycard: TopupStatsByCard,
}

impl TopupServiceImpl {
    pub fn new(
        query: DynTopupQueryService,
        command: DynTopupCommandService,
        stats: TopupStats,
        statsbycard: TopupStatsByCard,
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
impl TopupService for TopupServiceImpl {
    async fn find_all_topup(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopup {
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

    async fn find_all_topup_by_card_number(
        &self,
        request: Request<FindAllTopupByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTopupsByCardNumber {
            card_number: req.card_number,
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all_by_card_number(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopup {
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

    async fn find_by_id_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();

        match self.query.find_by_id(req.topup_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_status_success(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthTopupStatus {
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
                let grpc_response = ApiResponseTopupMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_status_success(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_success(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_status_failed(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthTopupStatus {
            year: req.year,
            month: req.month,
        };

        match self.stats.status.get_month_status_failed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_status_failed(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_failed(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthTopupStatusCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_status_success_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = YearTopupStatusCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthTopupStatusCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = YearTopupStatusCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
        let req = request.into_inner();

        match self.stats.method.get_monthly_methods(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
        let req = request.into_inner();

        match self.stats.method.get_yearly_methods(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_monthly_amounts(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_yearly_amounts(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .method
            .get_monthly_methods(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .method
            .get_yearly_methods(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_monthly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
        let req = request.into_inner();
        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_yearly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_card_number_topup(
        &self,
        request: Request<FindByCardNumberTopupRequest>,
    ) -> Result<Response<ApiResponsesTopup>, Status> {
        let req = request.into_inner();

        match self.query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsesTopup {
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
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopupDeleteAt {
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
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopupDeleteAt {
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

    async fn create_topup(
        &self,
        request: Request<CreateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();
        let domain_req = DomainCreateTopupRequest {
            card_number: req.card_number,
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_topup(
        &self,
        request: Request<UpdateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();
        let domain_req = DomainUpdateTopupRequst {
            card_number: req.card_number,
            topup_id: req.topup_id,
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trashed(req.topup_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.topup_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_topup_permanent(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete_permanent(req.topup_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_topup(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_topup_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
