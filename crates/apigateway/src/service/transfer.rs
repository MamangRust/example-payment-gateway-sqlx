use async_trait::async_trait;
use genproto::transfer::{
    CreateTransferRequest, FindAllTransferRequest, FindByCardNumberTransferRequest,
    FindByIdTransferRequest, FindMonthlyTransferStatus, FindMonthlyTransferStatusCardNumber,
    FindTransferByTransferFromRequest, FindTransferByTransferToRequest, FindYearTransferStatus,
    FindYearTransferStatusCardNumber, UpdateTransferRequest,
    transfer_service_client::TransferServiceClient,
};
use shared::{
    abstract_trait::transfer::http::{
        command::TransferCommandGrpcClientTrait,
        query::TransferQueryGrpcClientTrait,
        stats::{
            amount::TransferStatsAmountGrpcClientTrait, status::TransferStatsStatusGrpcClientTrait,
        },
        statsbycard::{
            amount::TransferStatsAmountByCardNumberGrpcClientTrait,
            status::TransferStatsStatusByCardNumberGrpcClientTrait,
        },
    },
    domain::{
        requests::transfer::{
            CreateTransferRequest as DomainCreateTransferRequest,
            FindAllTransfers as DomainFindAllTransfers,
            MonthStatusTransfer as DomainMonthStatusTransfer,
            MonthStatusTransferCardNumber as DomainMonthStatusTransferCardNumber,
            MonthYearCardNumber as DomainMonthYearCardNumber,
            UpdateTransferRequest as DomainUpdateTransferRequest,
            YearStatusTransferCardNumber as DomainYearStatusTransferCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransferMonthAmountResponse, TransferResponse,
            TransferResponseDeleteAt, TransferResponseMonthStatusFailed,
            TransferResponseMonthStatusSuccess, TransferResponseYearStatusFailed,
            TransferResponseYearStatusSuccess, TransferYearAmountResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::{mask_card_number, month_name},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[async_trait]
#[allow(dead_code)]
pub trait TransferGrpcClientServiceTrait:
    TransferCommandGrpcClientTrait
    + TransferQueryGrpcClientTrait
    + TransferStatsAmountGrpcClientTrait
    + TransferStatsStatusGrpcClientTrait
    + TransferStatsAmountByCardNumberGrpcClientTrait
    + TransferStatsStatusByCardNumberGrpcClientTrait
{
}

#[derive(Debug)]
pub struct TransferGrpcClientService {
    client: Arc<Mutex<TransferServiceClient<Channel>>>,
}

impl TransferGrpcClientService {
    pub async fn new(client: Arc<Mutex<TransferServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TransferQueryGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponse>>, AppErrorHttp> {
        info!(
            "fetching all transfers - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransferRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} transfers", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all transfers failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp> {
        info!("fetching transfer by id: {transfer_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        match client.find_by_id_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transfer {transfer_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transfer data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found transfer {transfer_id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find transfer {transfer_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active transfers - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransferRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active transfers", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active transfers failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllTransfers,
    ) -> Result<ApiResponsePagination<Vec<TransferResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed transfers - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransferRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed transfers", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed transfers failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, transfer_from), level = "info")]
    async fn find_by_transfer_from(
        &self,
        transfer_from: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, AppErrorHttp> {
        let masked_from = mask_card_number(transfer_from);
        info!("fetching transfers FROM card: {masked_from}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindTransferByTransferFromRequest {
            transfer_from: transfer_from.to_string(),
        });

        match client.find_transfer_by_transfer_from(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} transfers from card {masked_from}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch transfers FROM card {masked_from} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, transfer_to), level = "info")]
    async fn find_by_transfer_to(
        &self,
        transfer_to: &str,
    ) -> Result<ApiResponse<Vec<TransferResponse>>, AppErrorHttp> {
        let masked_to = mask_card_number(transfer_to);
        info!("fetching transfers TO card: {masked_to}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindTransferByTransferToRequest {
            transfer_to: transfer_to.to_string(),
        });

        match client.find_transfer_by_transfer_to(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} transfers to card {masked_to}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch transfers TO card {masked_to} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferCommandGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp> {
        let masked_from = mask_card_number(&req.transfer_from);
        let masked_to = mask_card_number(&req.transfer_to);
        info!(
            "creating transfer FROM {masked_from} TO {masked_to}, amount: {}",
            req.transfer_amount
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(CreateTransferRequest {
            transfer_from: req.transfer_from.clone(),
            transfer_to: req.transfer_to.clone(),
            transfer_amount: req.transfer_amount as i32,
        });

        match client.create_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transfer creation failed - data missing in gRPC response FROM {masked_from} TO {masked_to}");
                    AppErrorHttp(AppErrorGrpc::Unhandled("Transfer data is missing in gRPC response".into()))
                })?;

                info!("transfer created successfully FROM {masked_from} TO {masked_to}");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create transfer FROM {masked_from} TO {masked_to} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, AppErrorHttp> {
        let masked_from = mask_card_number(&req.transfer_from);
        let masked_to = mask_card_number(&req.transfer_to);
        info!(
            "updating transfer id: {} FROM {masked_from} TO {masked_to}, new amount: {}",
            req.transfer_id, req.transfer_amount
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(UpdateTransferRequest {
            transfer_id: req.transfer_id,
            transfer_from: req.transfer_from.clone(),
            transfer_to: req.transfer_to.clone(),
            transfer_amount: req.transfer_amount as i32,
        });

        match client.update_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "update transfer {} - data missing in gRPC response",
                        req.transfer_id
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transfer data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "transfer {} updated successfully FROM {masked_from} TO {masked_to}",
                    req.transfer_id
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update transfer {} failed: {status:?}", req.transfer_id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, AppErrorHttp> {
        info!("trashing transfer id: {transfer_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        match client.trashed_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash transfer {transfer_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transfer data is missing in gRPC response".into(),
                    ))
                })?;

                info!("transfer {transfer_id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash transfer {transfer_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        transfer_id: i32,
    ) -> Result<ApiResponse<TransferResponseDeleteAt>, AppErrorHttp> {
        info!("restoring transfer id: {transfer_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        match client.restore_transfer(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore transfer {transfer_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transfer data is missing in gRPC response".into(),
                    ))
                })?;

                info!("transfer {transfer_id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore transfer {transfer_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(&self, transfer_id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting transfer id: {transfer_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransferRequest { transfer_id });

        match client.delete_transfer_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("transfer {transfer_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete transfer {transfer_id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed transfers");

        let mut client = self.client.lock().await;

        match client.restore_all_transfer(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed transfers restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all transfers failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all transfers");

        let mut client = self.client.lock().await;

        match client.delete_all_transfer_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all transfers permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all transfers permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferStatsAmountGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp> {
        info!("fetching monthly transfer AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatus { year });

        match client.find_monthly_transfer_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transfer amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly transfer AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp> {
        info!("fetching yearly transfer AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatus { year });

        match client.find_yearly_transfer_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transfer amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly transfer AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferStatsStatusGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer SUCCESS status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransferStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_transfer_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS transfer records for {month_str} {}",
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
                    "fetch monthly SUCCESS transfer status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, AppErrorHttp> {
        info!("fetching yearly transfer SUCCESS status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatus { year });

        match client.find_yearly_transfer_status_success(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS transfer records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly SUCCESS transfer status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusTransfer,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer FAILED status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransferStatus {
            year: req.year,
            month: req.month,
        });

        match client.find_monthly_transfer_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED transfer records for {month_str} {}",
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
                    "fetch monthly FAILED transfer status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, AppErrorHttp> {
        info!("fetching yearly transfer FAILED status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatus { year });

        match client.find_yearly_transfer_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED transfer records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly FAILED transfer status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferStatsAmountByCardNumberGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_by_sender(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transfer AMOUNT as SENDER for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_transfer_amounts_by_sender_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transfer amount records as SENDER for card {masked_card} year {}",
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
                    "fetch monthly transfer AMOUNT as SENDER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_by_receiver(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferMonthAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transfer AMOUNT as RECEIVER for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_transfer_amounts_by_receiver_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transfer amount records as RECEIVER for card {masked_card} year {}",
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
                    "fetch monthly transfer AMOUNT as RECEIVER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_by_sender(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer AMOUNT as SENDER for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transfer_amounts_by_sender_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transfer amount records as SENDER for card {masked_card} year {}",
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
                    "fetch yearly transfer AMOUNT as SENDER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_by_receiver(
        &self,
        req: &DomainMonthYearCardNumber,
    ) -> Result<ApiResponse<Vec<TransferYearAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer AMOUNT as RECEIVER for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByCardNumberTransferRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transfer_amounts_by_receiver_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferYearAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transfer amount records as RECEIVER for card {masked_card} year {}",
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
                    "fetch yearly transfer AMOUNT as RECEIVER for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferStatsStatusByCardNumberGrpcClientTrait for TransferGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_by_card(
        &self,
        req: &DomainMonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transfer_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS transfer records for card {masked_card} {month_str} {}",
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
                    "fetch monthly SUCCESS transfer status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_by_card(
        &self,
        req: &DomainYearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transfer_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS transfer records for card {masked_card} year {}",
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
                    "fetch yearly SUCCESS transfer status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_by_card(
        &self,
        req: &DomainMonthStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseMonthStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transfer FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transfer_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED transfer records for card {masked_card} {month_str} {}",
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
                    "fetch monthly FAILED transfer status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_by_card(
        &self,
        req: &DomainYearStatusTransferCardNumber,
    ) -> Result<ApiResponse<Vec<TransferResponseYearStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transfer FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransferStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transfer_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransferResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED transfer records for card {masked_card} year {}",
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
                    "fetch yearly FAILED transfer status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransferGrpcClientServiceTrait for TransferGrpcClientService {}
