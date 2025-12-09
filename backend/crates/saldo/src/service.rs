use crate::di::DependenciesInject;
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
    errors::AppErrorGrpc,
    utils::mask_card_number,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};


#[derive(Clone)]
pub struct SaldoServiceImpl {
    pub di: Arc<DependenciesInject>,
}

impl SaldoServiceImpl {
    pub fn new(di: Arc<DependenciesInject>) -> Self {
        Self { di }
    }
}

#[tonic::async_trait]
impl SaldoService for SaldoServiceImpl {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all_saldo(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldo>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_all_saldo - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.saldo_query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_all_saldo succeeded - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_all_saldo failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_id_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();
        info!("handling find_by_id_saldo - saldo_id: {}", req.saldo_id);

        match self.di.saldo_query.find_by_id(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("find_by_id_saldo succeeded for id: {}", req.saldo_id);
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_by_id_saldo failed for id {}: {e:?}", req.saldo_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_total_saldo_balance(
        &self,
        request: Request<FindMonthlySaldoTotalBalance>,
    ) -> Result<Response<ApiResponseMonthTotalSaldo>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_monthly_total_saldo_balance - year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthTotalSaldoBalance {
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .saldo_total_balance
            .get_month_total_balance(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMonthTotalSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_total_saldo_balance succeeded - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_total_saldo_balance failed for year {} month {}: {e:?}",
                    req.year, req.month
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_year_total_saldo_balance(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearTotalSaldo>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_year_total_saldo_balance - year: {}",
            req.year
        );

        match self
            .di
            .saldo_total_balance
            .get_year_total_balance(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseYearTotalSaldo {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_year_total_saldo_balance succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_year_total_saldo_balance failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseMonthSaldoBalances>, Status> {
        let req = request.into_inner();
        info!("handling find_monthly_saldo_balances - year: {}", req.year);

        match self.di.saldo_balance.get_month_balance(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMonthSaldoBalances {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_saldo_balances succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_saldo_balances failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_saldo_balances(
        &self,
        request: Request<FindYearlySaldo>,
    ) -> Result<Response<ApiResponseYearSaldoBalances>, Status> {
        let req = request.into_inner();
        info!("handling find_yearly_saldo_balances - year: {}", req.year);

        match self.di.saldo_balance.get_year_balance(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseYearSaldoBalances {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_yearly_saldo_balances succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_yearly_saldo_balances failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);

        info!("handling find_by_card_number - card: {masked_card}");

        match self.di.saldo_query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("find_by_card_number succeeded for card: {masked_card}");
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_by_card_number failed for card {masked_card}: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_by_active - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.saldo_query.find_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldoDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_by_active succeeded - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_by_active failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip(self, request), level = "info")]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllSaldoRequest>,
    ) -> Result<Response<ApiResponsePaginationSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_by_trashed - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllSaldos {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.saldo_query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationSaldoDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_by_trashed succeeded - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_by_trashed failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn create_saldo(
        &self,
        request: Request<CreateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling create_saldo - card: {masked_card}, balance: {}",
            req.total_balance
        );

        let domain_req = DomainCreateSaldoRequest {
            card_number: req.card_number,
            total_balance: req.total_balance as i64,
        };

        match self.di.saldo_command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("create_saldo succeeded for card: {masked_card}");
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("create_saldo failed for card {masked_card}: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update_saldo(
        &self,
        request: Request<UpdateSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldo>, Status> {
        let req = request.into_inner();
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling update_saldo - saldo_id: {}, card: {masked_card}, new balance: {}",
            req.saldo_id, req.total_balance
        );

        let domain_req = DomainUpdateSaldoRequest {
            saldo_id: Some(req.saldo_id),
            card_number: req.card_number,
            total_balance: req.total_balance as i64,
        };

        match self.di.saldo_command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldo {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "update_saldo succeeded for id: {} card: {masked_card}",
                    req.saldo_id
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "update_saldo failed for id {} card {masked_card}: {e:?}",
                    req.saldo_id
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn trashed_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        info!("handling trashed_saldo - saldo_id: {}", req.saldo_id);

        match self.di.saldo_command.trash(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("trashed_saldo succeeded for id: {}", req.saldo_id);
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("trashed_saldo failed for id {}: {e:?}", req.saldo_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn restore_saldo(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDeleteAt>, Status> {
        let req = request.into_inner();
        info!("handling restore_saldo - saldo_id: {}", req.saldo_id);

        match self.di.saldo_command.restore(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("restore_saldo succeeded for id: {}", req.saldo_id);
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("restore_saldo failed for id {}: {e:?}", req.saldo_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn delete_saldo_permanent(
        &self,
        request: Request<FindByIdSaldoRequest>,
    ) -> Result<Response<ApiResponseSaldoDelete>, Status> {
        let req = request.into_inner();
        info!(
            "handling delete_saldo_permanent - saldo_id: {}",
            req.saldo_id
        );

        match self.di.saldo_command.delete(req.saldo_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoDelete {
                    status: api_response.status,
                    message: api_response.message,
                };
                info!("delete_saldo_permanent succeeded for id: {}", req.saldo_id);
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "delete_saldo_permanent failed for id {}: {e:?}",
                    req.saldo_id
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), level = "info")]
    async fn restore_all_saldo(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        info!("handling restore_all_saldo");

        match self.di.saldo_command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                };
                info!("restore_all_saldo succeeded");
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("restore_all_saldo failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, _request), level = "info")]
    async fn delete_all_saldo_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseSaldoAll>, Status> {
        info!("handling delete_all_saldo_permanent");

        match self.di.saldo_command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseSaldoAll {
                    status: api_response.status,
                    message: api_response.message,
                };
                info!("delete_all_saldo_permanent succeeded");
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("delete_all_saldo_permanent failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
