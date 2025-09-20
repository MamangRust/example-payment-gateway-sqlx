use async_trait::async_trait;
use genproto::withdraw::{
    CreateWithdrawRequest, FindAllWithdrawByCardNumberRequest, FindAllWithdrawRequest,
    FindByIdWithdrawRequest, FindMonthlyWithdrawStatus, FindMonthlyWithdrawStatusCardNumber,
    FindYearWithdrawCardNumber, FindYearWithdrawStatus, FindYearWithdrawStatusCardNumber,
    UpdateWithdrawRequest, withdraw_service_client::WithdrawServiceClient,
};

use shared::{
    abstract_trait::withdraw::http::{
        WithdrawCommandGrpcClientTrait, WithdrawGrpcClientServiceTrait,
        WithdrawQueryGrpcClientTrait, WithdrawStatsAmountByCardNumberGrpcClientTrait,
        WithdrawStatsAmountGrpcClientTrait, WithdrawStatsStatusByCardNumberGrpcClientTrait,
        WithdrawStatsStatusGrpcClientTrait,
    },
    domain::{
        requests::withdraw::{
            CreateWithdrawRequest as DomainCreateWithdrawRequest,
            FindAllWithdrawCardNumber as DomainFindAllWithdrawCardNumber,
            FindAllWithdraws as DomainFindAllWithdraws,
            MonthStatusWithdraw as DomainMonthStatusWithdraw,
            MonthStatusWithdrawCardNumber as DomainMonthStatusWithdrawCardNumber,
            UpdateWithdrawRequest as DomainUpdateWithdrawRequest,
            YearMonthCardNumber as DomainYearMonthCardNumber,
            YearStatusWithdrawCardNumber as DomainYearStatusWithdrawCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, WithdrawMonthlyAmountResponse, WithdrawResponse,
            WithdrawResponseDeleteAt, WithdrawResponseMonthStatusFailed,
            WithdrawResponseMonthStatusSuccess, WithdrawResponseYearStatusFailed,
            WithdrawResponseYearStatusSuccess, WithdrawYearlyAmountResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::{mask_card_number, month_name, naive_datetime_to_timestamp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct WithdrawGrpcClientService {
    client: Arc<Mutex<WithdrawServiceClient<Channel>>>,
}

impl WithdrawGrpcClientService {
    pub async fn new(client: Arc<Mutex<WithdrawServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl WithdrawGrpcClientServiceTrait for WithdrawGrpcClientService {}

#[async_trait]
impl WithdrawQueryGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, AppErrorHttp> {
        info!(
            "fetching all withdraws - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllWithdrawRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} withdraws", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all withdraws failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &DomainFindAllWithdrawCardNumber,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching withdraws for card: {} - page: {}, page_size: {}, search: {:?}",
            masked_card, req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllWithdrawByCardNumberRequest {
            card_number: req.card_number.clone(),
            search: req.search.clone(),
            page: req.page,
            page_size: req.page_size,
        });

        match client.find_all_withdraw_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} withdraws for card {}", data.len(), masked_card);
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!(
                    "fetch withdraws for card {} failed: {status:?}",
                    masked_card
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponse>, AppErrorHttp> {
        info!("fetching withdraw by id: {withdraw_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        match client.find_by_id_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("withdraw {withdraw_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Withdraw data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found withdraw {withdraw_id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find withdraw {withdraw_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active withdraws - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllWithdrawRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active withdraws", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active withdraws failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllWithdraws,
    ) -> Result<ApiResponsePagination<Vec<WithdrawResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed withdraws - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllWithdrawRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed withdraws", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed withdraws failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl WithdrawCommandGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating withdraw for card: {masked_card}, amount: {}",
            req.withdraw_amount
        );

        let mut client = self.client.lock().await;

        let date = naive_datetime_to_timestamp(req.withdraw_time);

        let grpc_req = Request::new(CreateWithdrawRequest {
            card_number: req.card_number.clone(),
            withdraw_amount: req.withdraw_amount,
            withdraw_time: Some(date),
        });

        match client.create_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("withdraw creation failed - data missing in gRPC response for card: {masked_card}");
                    AppErrorHttp(AppErrorGrpc::Unhandled("Withdraw data is missing in gRPC response".into()))
                })?;

                info!("withdraw created successfully for card: {masked_card}");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create withdraw for card {masked_card} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);

        let withdraw_id = req.withdraw_id.ok_or_else(|| {
            AppErrorHttp(AppErrorGrpc::Unhandled(
                "widhdraw_id is required".to_string(),
            ))
        })?;

        info!(
            "updating withdraw id: {withdraw_id} for card: {masked_card}, new amount: {}",
            req.withdraw_amount
        );

        let mut client = self.client.lock().await;

        let date = naive_datetime_to_timestamp(req.withdraw_time);

        let grpc_req = Request::new(UpdateWithdrawRequest {
            card_number: req.card_number.clone(),
            withdraw_id: withdraw_id,
            withdraw_amount: req.withdraw_amount,
            withdraw_time: Some(date),
        });

        match client.update_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "update withdraw {} - data missing in gRPC response",
                        withdraw_id
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Withdraw data is missing in gRPC response".into(),
                    ))
                })?;

                info!("withdraw {withdraw_id} updated successfully for card: {masked_card}",);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update withdraw {withdraw_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, AppErrorHttp> {
        info!("trashing withdraw id: {withdraw_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        match client.trashed_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash withdraw {withdraw_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Withdraw data is missing in gRPC response".into(),
                    ))
                })?;

                info!("withdraw {withdraw_id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash withdraw {withdraw_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, AppErrorHttp> {
        info!("restoring withdraw id: {withdraw_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        match client.restore_withdraw(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore withdraw {withdraw_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Withdraw data is missing in gRPC response".into(),
                    ))
                })?;

                info!("withdraw {withdraw_id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore withdraw {withdraw_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting withdraw id: {withdraw_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdWithdrawRequest { withdraw_id });

        match client.delete_withdraw_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("withdraw {withdraw_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete withdraw {withdraw_id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed withdraws");

        let mut client = self.client.lock().await;

        match client.restore_all_withdraw(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed withdraws restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all withdraws failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all withdraws");

        let mut client = self.client.lock().await;

        match client.delete_all_withdraw_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all withdraws permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all withdraws permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsAmountGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp> {
        info!("fetching monthly withdraw AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatus { year });

        match client.find_monthly_withdraws(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawMonthlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly withdraw amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly withdraw AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_withdraws(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp> {
        info!("fetching yearly withdraw AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatus { year });

        match client.find_yearly_withdraws(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly withdraw amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly withdraw AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsStatusGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw SUCCESS status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyWithdrawStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_withdraw_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS withdraw records for {month_str} {}",
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
                    "fetch monthly SUCCESS withdraw status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, AppErrorHttp> {
        info!("fetching yearly withdraw SUCCESS status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatus { year });

        match client.find_yearly_withdraw_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS withdraw records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly SUCCESS withdraw status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusWithdraw,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw FAILED status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyWithdrawStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_withdraw_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED withdraw records for {month_str} {}",
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
                    "fetch monthly FAILED withdraw status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, AppErrorHttp> {
        info!("fetching yearly withdraw FAILED status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatus { year });

        match client.find_yearly_withdraw_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED withdraw records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly FAILED withdraw status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsAmountByCardNumberGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_bycard(
        &self,
        req: &DomainYearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawMonthlyAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly withdraw AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_monthly_withdraws_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawMonthlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly withdraw amount records for card {masked_card} year {}",
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
                    "fetch monthly withdraw AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_bycard(
        &self,
        req: &DomainYearMonthCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawYearlyAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_yearly_withdraws_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly withdraw amount records for card {masked_card} year {}",
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
                    "fetch yearly withdraw AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl WithdrawStatsStatusByCardNumberGrpcClientTrait for WithdrawGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_withdraw_status_success_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS withdraw records for card {masked_card} {month_str} {}",
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
                    "fetch monthly SUCCESS withdraw status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_withdraw_status_success_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS withdraw records for card {masked_card} year {}",
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
                    "fetch yearly SUCCESS withdraw status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseMonthStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly withdraw FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_withdraw_status_failed_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED withdraw records for card {masked_card} {month_str} {}",
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
                    "fetch monthly FAILED withdraw status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearStatusWithdrawCardNumber,
    ) -> Result<ApiResponse<Vec<WithdrawResponseYearStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly withdraw FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearWithdrawStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_withdraw_status_failed_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<WithdrawResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED withdraw records for card {masked_card} year {}",
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
                    "fetch yearly FAILED withdraw status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}
