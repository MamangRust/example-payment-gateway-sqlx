use async_trait::async_trait;
use std::sync::Arc;

use genproto::saldo::{
    CreateSaldoRequest, FindAllSaldoRequest, FindByIdSaldoRequest, FindMonthlySaldoTotalBalance,
    FindYearlySaldo, UpdateSaldoRequest, saldo_service_client::SaldoServiceClient,
};
use shared::{
    abstract_trait::saldo::http::{
        command::SaldoCommandGrpcClientTrait,
        query::SaldoQueryGrpcClientTrait,
        stats::{balance::SaldoBalanceGrpcClientTrait, total::SaldoTotalBalanceGrpcClientTrait},
    },
    domain::{
        requests::saldo::{
            CreateSaldoRequest as DomainCreateSaldoRequest, FindAllSaldos as DomainFindAllSaldos,
            MonthTotalSaldoBalance as DomainMonthTotalSaldoBalance,
            UpdateSaldoRequest as DomainUpdateSaldoRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, SaldoMonthBalanceResponse,
            SaldoMonthTotalBalanceResponse, SaldoResponse, SaldoResponseDeleteAt,
            SaldoYearBalanceResponse, SaldoYearTotalBalanceResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::{mask_card_number, month_name},
};
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[async_trait]
#[allow(dead_code)]
pub trait SaldoGrpcClientServiceTrait:
    SaldoQueryGrpcClientTrait
    + SaldoCommandGrpcClientTrait
    + SaldoBalanceGrpcClientTrait
    + SaldoTotalBalanceGrpcClientTrait
{
}

#[derive(Debug)]
pub struct SaldoGrpcClientService {
    client: Arc<Mutex<SaldoServiceClient<Channel>>>,
}

impl SaldoGrpcClientService {
    pub async fn new(client: Arc<Mutex<SaldoServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SaldoQueryGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponse>>, AppErrorHttp> {
        info!(
            "fetching all saldos - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllSaldoRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} saldos", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all saldos failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_active(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active saldos - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllSaldoRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active saldos", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active saldos failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_trashed(
        &self,
        request: &DomainFindAllSaldos,
    ) -> Result<ApiResponsePagination<Vec<SaldoResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed saldos - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllSaldoRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed saldos", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed saldos failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<SaldoResponse>, AppErrorHttp> {
        info!("fetching saldo by id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdSaldoRequest { saldo_id: id });

        match client.find_by_id_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("saldo {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Saldo data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found saldo {id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find saldo {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl SaldoCommandGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: &DomainCreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&request.card_number);

        info!(
            "creating saldo for card: {masked_card} with balance: {}",
            request.total_balance
        );

        let mut client = self.client.lock().await;

        let grpc_req = CreateSaldoRequest {
            card_number: request.card_number.clone(),
            total_balance: request.total_balance as i32,
        };

        match client.create_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("saldo creation failed - data missing in gRPC response for card: {masked_card}");
                    AppErrorHttp(AppErrorGrpc::Unhandled("Saldo data is missing in gRPC response".into()))
                })?;

                info!("saldo created successfully for card: {masked_card}");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create saldo for card {masked_card} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: &DomainUpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&request.card_number);
        info!(
            "updating saldo id: {} for card: {} with new balance: {}",
            request.saldo_id, masked_card, request.total_balance
        );

        let mut client = self.client.lock().await;

        let grpc_req = UpdateSaldoRequest {
            saldo_id: request.saldo_id,
            card_number: request.card_number.clone(),
            total_balance: request.total_balance as i32,
        };

        match client.update_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "update saldo {} - data missing in gRPC response",
                        request.saldo_id
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Saldo data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "saldo {} updated successfully for card: {}",
                    request.saldo_id, masked_card
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update saldo {} failed: {status:?}", request.saldo_id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, AppErrorHttp> {
        info!("trashing saldo id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdSaldoRequest { saldo_id: id };

        match client.trashed_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash saldo {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Saldo data is missing in gRPC response".into(),
                    ))
                })?;

                info!("saldo {id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash saldo {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, AppErrorHttp> {
        info!("restoring saldo id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdSaldoRequest { saldo_id: id };

        match client.restore_saldo(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore saldo {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Saldo data is missing in gRPC response".into(),
                    ))
                })?;

                info!("saldo {id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore saldo {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting saldo id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdSaldoRequest { saldo_id: id };

        match client.delete_saldo_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("saldo {id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete saldo {id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed saldos");

        let mut client = self.client.lock().await;

        match client.restore_all_saldo(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed saldos restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all saldos failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all saldos");

        let mut client = self.client.lock().await;

        match client.delete_all_saldo_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all saldos permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all saldos permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl SaldoBalanceGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_month_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoMonthBalanceResponse>>, AppErrorHttp> {
        info!("fetching monthly BALANCE for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearlySaldo { year });

        match client.find_monthly_saldo_balances(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoMonthBalanceResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly balance records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly BALANCE for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_year_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearBalanceResponse>>, AppErrorHttp> {
        info!("fetching yearly BALANCE for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearlySaldo { year });

        match client.find_yearly_saldo_balances(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoYearBalanceResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly balance records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly BALANCE for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl SaldoTotalBalanceGrpcClientTrait for SaldoGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_total_balance(
        &self,
        req: &DomainMonthTotalSaldoBalance,
    ) -> Result<ApiResponse<Vec<SaldoMonthTotalBalanceResponse>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly TOTAL BALANCE for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlySaldoTotalBalance {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_total_saldo_balance(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoMonthTotalBalanceResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly total balance records for {month_str} {}",
                    data.len(),
                    req.year
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!(
                    "fetch monthly TOTAL BALANCE for {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_year_total_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<SaldoYearTotalBalanceResponse>>, AppErrorHttp> {
        info!("fetching yearly TOTAL BALANCE for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearlySaldo { year });

        match client.find_year_total_saldo_balance(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<SaldoYearTotalBalanceResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly total balance records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly TOTAL BALANCE for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}
