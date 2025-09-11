use genproto::{
    card::FindByCardNumberRequest,
    saldo::{
        ApiResponseMonthSaldoBalances, ApiResponseMonthTotalSaldo, ApiResponsePaginationSaldo,
        ApiResponsePaginationSaldoDeleteAt, ApiResponseSaldo, ApiResponseSaldoAll,
        ApiResponseSaldoDelete, ApiResponseSaldoDeleteAt, ApiResponseYearSaldoBalances,
        ApiResponseYearTotalSaldo, ApiResponsesSaldo, CreateSaldoRequest, FindAllSaldoRequest,
        FindByIdSaldoRequest, FindMonthlySaldoTotalBalance, FindYearlySaldo, UpdateSaldoRequest,
        saldo_service_server::SaldoService,
    },
};
use shared::{
    abstract_trait::saldo::service::{
        command::DynSaldoCommandService,
        query::DynSaldoQueryService,
        stats::{balance::DynSaldoBalanceService, total::DynSaldoTotalBalanceService},
    },
    domain::requests::saldo::{
        CreateSaldoRequest as DomainCreateSaldoRequest, FindAllSaldos, MonthTotalSaldoBalance,
        UpdateSaldoRequest as DomainUpdateSaldoRequest,
    },
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct SaldoStats {
    pub balance: DynSaldoBalanceService,
    pub total: DynSaldoTotalBalanceService,
}

#[derive(Clone)]
pub struct SaldoServiceImpl {
    pub query: DynSaldoQueryService,
    pub command: DynSaldoCommandService,
    pub stats: SaldoStats,
}

#[tonic::async_trait]
impl SaldoService for SaldoServiceImpl {
    async fn find_all_saldo(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldo>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldo {
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

    async fn find_by_id_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();

        match self.query.find_by_id(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_total_saldo_balance(
        &self,
        request: Request<FindMonthlySaldoTotalBalance>,
    ) -> Result<Response<ApiResponseMonthTotalSaldo>, Status> {
        let req = request.into_inner();
        let domain_req = MonthTotalSaldoBalance {
            year: req.year,
            month: req.month,
        };

        match self.stats.total.get_month_total_balance(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMonthTotalSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_year_total_saldo_balance(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearTotalSaldo>, Status> {
        let req = request.into_inner();

        match self.stats.total.get_year_total_balance(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseYearTotalSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseMonthSaldoBalances>, Status> {
        let req = request.into_inner();

        match self.stats.balance.get_month_balance(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMonthSaldoBalances {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearSaldoBalances>, Status> {
        let req = request.into_inner();

        match self.stats.balance.get_year_balance(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseYearSaldoBalances {
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
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();

        match self.query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
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
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldoDeleteAt {
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
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldoDeleteAt {
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

    async fn create_saldo(
        &self,
        request: Request<CreateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();
        let domain_req = DomainCreateSaldoRequest {
            card_number: req.card_number,
            total_balance: req.total_balance as i64,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_saldo(
        &self,
        request: Request<UpdateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();
        let domain_req = DomainUpdateSaldoRequest {
            saldo_id: req.saldo_id,
            card_number: req.card_number,
            total_balance: req.total_balance as i64,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trash(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_saldo_permanent(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDelete {
                    status: api_response.status,
                    message: api_response.message,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_saldo(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_saldo_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
