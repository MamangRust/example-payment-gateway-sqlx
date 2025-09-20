use async_trait::async_trait;
use genproto::topup::{
    CreateTopupRequest, FindAllTopupByCardNumberRequest, FindAllTopupRequest,
    FindByCardNumberTopupRequest, FindByIdTopupRequest, FindMonthlyTopupStatus,
    FindMonthlyTopupStatusCardNumber, FindYearTopupCardNumber, FindYearTopupStatus,
    FindYearTopupStatusCardNumber, UpdateTopupRequest, topup_service_client::TopupServiceClient,
};
use shared::{
    abstract_trait::topup::http::{
        TopupCommandGrpcClientTrait, TopupGrpcClientServiceTrait, TopupQueryGrpcClientTrait,
        TopupStatsAmountByCardNumberGrpcClientTrait, TopupStatsAmountGrpcClientTrait,
        TopupStatsMethodByCardNumberGrpcClientTrait, TopupStatsMethodGrpcClientTrait,
        TopupStatsStatusByCardNumberGrpcClientTrait, TopupStatsStatusGrpcClientTrait,
    },
    domain::{
        requests::topup::{
            CreateTopupRequest as DomainCreateTopupRequest, FindAllTopups as DomainFindAllTopups,
            FindAllTopupsByCardNumber as DomainFindAllTopupsByCardNumber,
            MonthTopupStatus as DomainMonthTopupStatus,
            MonthTopupStatusCardNumber as DomainMonthTopupStatusCardNumber,
            UpdateTopupRequest as DomainUpdateTopupRequest,
            YearMonthMethod as DomainYearMonthMethod,
            YearTopupStatusCardNumber as DomainYearTopupStatusCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TopupMonthAmountResponse, TopupMonthMethodResponse,
            TopupResponse, TopupResponseDeleteAt, TopupResponseMonthStatusFailed,
            TopupResponseMonthStatusSuccess, TopupResponseYearStatusFailed,
            TopupResponseYearStatusSuccess, TopupYearlyAmountResponse, TopupYearlyMethodResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::{mask_card_number, month_name},
};

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct TopupGrpcClientService {
    client: Arc<Mutex<TopupServiceClient<Channel>>>,
}

impl TopupGrpcClientService {
    pub async fn new(client: Arc<Mutex<TopupServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TopupGrpcClientServiceTrait for TopupGrpcClientService {}

#[async_trait]
impl TopupQueryGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, AppErrorHttp> {
        info!(
            "fetching all topups - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTopupRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} topups", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all topups failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &DomainFindAllTopupsByCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TopupResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching topups for card: {} - page: {}, page_size: {}, search: {:?}",
            masked_card, req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTopupByCardNumberRequest {
            card_number: req.card_number.clone(),
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_topup_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} topups for card {}", data.len(), masked_card);
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch topups for card {} failed: {status:?}", masked_card);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_active(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active topups - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTopupRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active topups", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active topups failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_trashed(
        &self,
        req: &DomainFindAllTopups,
    ) -> Result<ApiResponsePagination<Vec<TopupResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed topups - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTopupRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed topups", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed topups failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_card(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<Vec<TopupResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(card_number);
        info!("fetching topups by card: {masked_card}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByCardNumberTopupRequest {
            card_number: card_number.to_string(),
        });

        match client.find_by_card_number_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} topups for card {masked_card}", data.len());
                Ok(ApiResponse {
                    message: inner.message,
                    status: inner.status,
                    data,
                })
            }
            Err(status) => {
                error!("fetch topups by card {masked_card} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, topup_id: i32) -> Result<ApiResponse<TopupResponse>, AppErrorHttp> {
        info!("fetching topup by id: {topup_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        match client.find_by_id_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("topup {topup_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Topup data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found topup {topup_id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find topup {topup_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupCommandGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating topup for card: {masked_card}, amount: {}, method: {}",
            req.topup_amount, req.topup_method
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(CreateTopupRequest {
            card_number: req.card_number.clone(),
            topup_amount: req.topup_amount as i32,
            topup_method: req.topup_method.clone(),
        });

        match client.create_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("topup creation failed - data missing in gRPC response for card: {masked_card}");
                    AppErrorHttp(AppErrorGrpc::Unhandled("Topup data is missing in gRPC response".into()))
                })?;

                info!("topup created successfully for card: {masked_card}");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create topup for card {masked_card} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);

        let topup_id = req.topup_id.ok_or_else(|| {
            AppErrorHttp(AppErrorGrpc::Unhandled("topup_id is required".to_string()))
        })?;

        info!(
            "updating topup id: {topup_id} for card: {}, new amount: {}, method: {}",
            masked_card, req.topup_amount, req.topup_method
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(UpdateTopupRequest {
            card_number: req.card_number.clone(),
            topup_id: topup_id,
            topup_amount: req.topup_amount as i32,
            topup_method: req.topup_method.clone(),
        });

        match client.update_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update topup {topup_id} - data missing in gRPC response",);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Topup data is missing in gRPC response".into(),
                    ))
                })?;

                info!("topup {topup_id} updated successfully for card: {masked_card}",);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update topup {topup_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, AppErrorHttp> {
        info!("trashing topup id: {topup_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        match client.trashed_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash topup {topup_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Topup data is missing in gRPC response".into(),
                    ))
                })?;

                info!("topup {topup_id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash topup {topup_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, AppErrorHttp> {
        info!("restoring topup id: {topup_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        match client.restore_topup(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore topup {topup_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Topup data is missing in gRPC response".into(),
                    ))
                })?;

                info!("topup {topup_id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore topup {topup_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting topup id: {topup_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTopupRequest { topup_id });

        match client.delete_topup_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("topup {topup_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete topup {topup_id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed topups");

        let mut client = self.client.lock().await;

        match client.restore_all_topup(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed topups restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all topups failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_permanent(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all topups");

        let mut client = self.client.lock().await;

        match client.delete_all_topup_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all topups permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all topups permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsAmountGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp> {
        info!("fetching monthly topup AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_monthly_topup_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly topup AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp> {
        info!("fetching yearly topup AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_yearly_topup_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly topup AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsMethodGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp> {
        info!("fetching monthly topup METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_monthly_topup_methods(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly topup METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_methods(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp> {
        info!("fetching yearly topup METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_yearly_topup_methods(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupYearlyMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly topup METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsStatusGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup SUCCESS status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTopupStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_topup_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS topup records for {month_str} {}",
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
                    "fetch monthly SUCCESS topup status for {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_status_success(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, AppErrorHttp> {
        info!("fetching yearly topup SUCCESS status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_yearly_topup_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS topup records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly SUCCESS topup status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthTopupStatus,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly topup FAILED status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTopupStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_topup_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED topup records for {month_str} {}",
                    data.len(),
                    req.year,
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!(
                    "fetch monthly FAILED topup status for {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_status_failed(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, AppErrorHttp> {
        info!("fetching yearly topup FAILED status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupStatus { year });

        match client.find_yearly_topup_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED topup records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly FAILED topup status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsAmountByCardNumberGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly topup AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_topup_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup amount records for card {masked_card} year {}",
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
                    "fetch monthly topup AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_topup_amounts_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup amount records for card {masked_card} year {}",
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
                    "fetch yearly topup AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsMethodByCardNumberGrpcClientTrait for TopupGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_methods_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupMonthMethodResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly topup METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_topup_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup method records for card {masked_card} year {}",
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
                    "fetch monthly topup METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_methods_bycard(
        &self,
        req: &DomainYearMonthMethod,
    ) -> Result<ApiResponse<Vec<TopupYearlyMethodResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly topup METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTopupCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_topup_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TopupYearlyMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup method records for card {masked_card} year {}",
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
                    "fetch yearly topup METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TopupStatsStatusByCardNumberGrpcClientTrait for TopupGrpcClientService {
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusSuccess>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindMonthlyTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_topup_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();

                let data = inner.data.into_iter().map(Into::into).collect();

                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => Err(AppErrorHttp(AppErrorGrpc::from(status))),
        }
    }

    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusSuccess>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindYearTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_topup_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();

                let data = inner.data.into_iter().map(Into::into).collect();

                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => Err(AppErrorHttp(AppErrorGrpc::from(status))),
        }
    }

    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseMonthStatusFailed>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindMonthlyTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_topup_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();

                let data = inner.data.into_iter().map(Into::into).collect();

                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => Err(AppErrorHttp(AppErrorGrpc::from(status))),
        }
    }

    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearTopupStatusCardNumber,
    ) -> Result<ApiResponse<Vec<TopupResponseYearStatusFailed>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindYearTopupStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_topup_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();

                let data = inner.data.into_iter().map(Into::into).collect();

                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => Err(AppErrorHttp(AppErrorGrpc::from(status))),
        }
    }
}
