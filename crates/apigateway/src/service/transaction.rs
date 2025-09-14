use async_trait::async_trait;
use genproto::transaction::{
    CreateTransactionRequest, FindAllTransactionCardNumberRequest, FindAllTransactionRequest,
    FindByIdTransactionRequest, FindByYearCardNumberTransactionRequest,
    FindMonthlyTransactionStatus, FindMonthlyTransactionStatusCardNumber,
    FindTransactionByMerchantIdRequest, FindYearTransactionStatus,
    FindYearTransactionStatusCardNumber, UpdateTransactionRequest,
    transaction_service_client::TransactionServiceClient,
};
use shared::{
    abstract_trait::transaction::http::{
        TransactionCommandGrpcClientTrait, TransactionGrpcClientServiceTrait,
        TransactionQueryGrpcClientTrait, TransactionStatsAmountByCardNumberGrpcClientTrait,
        TransactionStatsAmountGrpcClientTrait, TransactionStatsMethodByCardNumberGrpcClientTrait,
        TransactionStatsMethodGrpcClientTrait, TransactionStatsStatusByCardNumberGrpcClientTrait,
        TransactionStatsStatusGrpcClientTrait,
    },
    domain::{
        requests::transaction::{
            CreateTransactionRequest as DomainCreateTransactionRequest,
            FindAllTransactionCardNumber, FindAllTransactions as DomainFindAllTransactions,
            MonthStatusTransaction as DomainMonthStatusTransaction,
            MonthStatusTransactionCardNumber as DomainMonthStatusTransactionCardNumber,
            MonthYearPaymentMethod as DomainMonthYearPaymentMethod,
            UpdateTransactionRequest as DomainUpdateTransactionRequest,
            YearStatusTransactionCardNumber as DomainYearStatusTransactionCardNumber,
        },
        responses::{
            ApiResponse, ApiResponsePagination, TransactionMonthAmountResponse,
            TransactionMonthMethodResponse, TransactionResponse, TransactionResponseDeleteAt,
            TransactionResponseMonthStatusFailed, TransactionResponseMonthStatusSuccess,
            TransactionResponseYearStatusFailed, TransactionResponseYearStatusSuccess,
            TransactionYearMethodResponse, TransactionYearlyAmountResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::{mask_api_key, mask_card_number, month_name, naive_datetime_to_timestamp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct TransactionGrpcClientService {
    client: Arc<Mutex<TransactionServiceClient<Channel>>>,
}

impl TransactionGrpcClientService {
    pub async fn new(client: Arc<Mutex<TransactionServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl TransactionGrpcClientServiceTrait for TransactionGrpcClientService {}

#[async_trait]
impl TransactionQueryGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, AppErrorHttp> {
        info!(
            "fetching all transactions - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransactionRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} transactions", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all transactions failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_all_by_card_number(
        &self,
        req: &FindAllTransactionCardNumber,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching transactions for card: {} - page: {}, page_size: {}, search: {:?}",
            masked_card, req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransactionCardNumberRequest {
            card_number: req.card_number.clone(),
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_transaction_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} transactions for card {}",
                    data.len(),
                    masked_card
                );
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!(
                    "fetch transactions for card {} failed: {status:?}",
                    masked_card
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_active(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active transactions - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransactionRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active transactions", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch active transactions failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_by_trashed(
        &self,
        req: &DomainFindAllTransactions,
    ) -> Result<ApiResponsePagination<Vec<TransactionResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed transactions - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllTransactionRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed transactions", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch trashed transactions failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp> {
        info!("fetching transaction by id: {transaction_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        match client.find_by_id_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transaction {transaction_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transaction data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found transaction {transaction_id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find transaction {transaction_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_merchant_id(
        &self,
        merchant_id: i32,
    ) -> Result<ApiResponse<Vec<TransactionResponse>>, AppErrorHttp> {
        info!("fetching transactions by merchant_id: {merchant_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindTransactionByMerchantIdRequest { merchant_id });

        match client.find_transaction_by_merchant_id(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} transactions for merchant {merchant_id}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch transactions for merchant {merchant_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionCommandGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, api_key, req), level = "info")]
    async fn create(
        &self,
        api_key: &str,
        req: &DomainCreateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp> {
        let masked_api = mask_api_key(api_key);
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "creating transaction via api_key: {masked_api} for card: {masked_card}, amount: {}, merchant_id: {:?}, method: {}",
            req.amount, req.merchant_id, req.payment_method
        );

        let mut client = self.client.lock().await;

        let date = naive_datetime_to_timestamp(req.transaction_time);

        let grpc_req = Request::new(CreateTransactionRequest {
            api_key: api_key.to_string(),
            card_number: req.card_number.clone(),
            amount: req.amount,
            payment_method: req.payment_method.clone(),
            merchant_id: req.merchant_id.unwrap_or(0),
            transaction_time: Some(date),
        });

        match client.create_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("transaction creation failed - data missing in gRPC response for card: {masked_card}");
                    AppErrorHttp(AppErrorGrpc::Unhandled("Transaction data is missing in gRPC response".into()))
                })?;

                info!("transaction created successfully for card: {masked_card}");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!(
                    "create transaction for card {masked_card} via api_key {masked_api} failed: {status:?}"
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, api_key, req), level = "info")]
    async fn update(
        &self,
        api_key: &str,
        req: &DomainUpdateTransactionRequest,
    ) -> Result<ApiResponse<TransactionResponse>, AppErrorHttp> {
        let masked_api = mask_api_key(api_key);
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "updating transaction id: {} via api_key: {masked_api} for card: {masked_card}, new amount: {}, method: {}",
            req.transaction_id, req.amount, req.payment_method
        );

        let mut client = self.client.lock().await;

        let date = naive_datetime_to_timestamp(req.transaction_time);

        let grpc_req = Request::new(UpdateTransactionRequest {
            transaction_id: req.transaction_id,
            api_key: api_key.to_string(),
            card_number: req.card_number.clone(),
            amount: req.amount as i32,
            payment_method: req.payment_method.clone(),
            merchant_id: req.merchant_id.unwrap_or(0),
            transaction_time: Some(date),
        });

        match client.update_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "update transaction {} - data missing in gRPC response",
                        req.transaction_id
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transaction data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "transaction {} updated successfully for card: {}",
                    req.transaction_id, masked_card
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!(
                    "update transaction {} via api_key {masked_api} failed: {status:?}",
                    req.transaction_id
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trashed(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, AppErrorHttp> {
        info!("trashing transaction id: {transaction_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        match client.trashed_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash transaction {transaction_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transaction data is missing in gRPC response".into(),
                    ))
                })?;

                info!("transaction {transaction_id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash transaction {transaction_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<TransactionResponseDeleteAt>, AppErrorHttp> {
        info!("restoring transaction id: {transaction_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdTransactionRequest { transaction_id });

        match client.restore_transaction(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore transaction {transaction_id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Transaction data is missing in gRPC response".into(),
                    ))
                })?;

                info!("transaction {transaction_id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore transaction {transaction_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_permanent(
        &self,
        transaction_id: i32,
    ) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting transaction id: {transaction_id}");

        let mut client = self.client.lock().await;

        let grpc_req = FindByIdTransactionRequest { transaction_id };

        match client.delete_transaction_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("transaction {transaction_id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete transaction {transaction_id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed transactions");

        let mut client = self.client.lock().await;

        match client.restore_all_transaction(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed transactions restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all transactions failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all transactions");

        let mut client = self.client.lock().await;

        match client.delete_all_transaction_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all transactions permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all transactions permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsAmountGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp> {
        info!("fetching monthly transaction AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client.find_monthly_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly transaction AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amounts(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp> {
        info!("fetching yearly transaction AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client.find_yearly_amounts(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly transaction AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsMethodGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, AppErrorHttp> {
        info!("fetching monthly transaction METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client.find_monthly_payment_methods(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly transaction METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, AppErrorHttp> {
        info!("fetching yearly transaction METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client.find_yearly_payment_methods(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionYearMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly transaction METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsStatusGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success(
        &self,
        req: &DomainMonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction SUCCESS status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransactionStatus {
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transaction_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS transaction records for {month_str} {}",
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
                    "fetch monthly SUCCESS transaction status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, AppErrorHttp> {
        info!("fetching yearly transaction SUCCESS status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client
            .find_yearly_transaction_status_success(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS transaction records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!(
                    "fetch yearly SUCCESS transaction status for year {year} failed: {status:?}"
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed(
        &self,
        req: &DomainMonthStatusTransaction,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, AppErrorHttp> {
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction FAILED status for {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransactionStatus {
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transaction_status_failed(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED transaction records for {month_str} {}",
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
                    "fetch monthly FAILED transaction status for {month_str} {} failed: {status:?}",
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
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, AppErrorHttp> {
        info!("fetching yearly transaction FAILED status for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatus { year });

        match client.find_yearly_transaction_status_failed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED transaction records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly FAILED transaction status for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsAmountByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amounts_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transaction AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_monthly_amounts_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionMonthAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction amount records for card {masked_card} year {}",
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
                    "fetch monthly transaction AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amounts_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearlyAmountResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction AMOUNT for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_yearly_amounts_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionYearlyAmountResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction amount records for card {masked_card} year {}",
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
                    "fetch yearly transaction AMOUNT for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsMethodByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionMonthMethodResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching monthly transaction METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_payment_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionMonthMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction method records for card {masked_card} year {}",
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
                    "fetch monthly transaction METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_bycard(
        &self,
        req: &DomainMonthYearPaymentMethod,
    ) -> Result<ApiResponse<Vec<TransactionYearMethodResponse>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction METHOD for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByYearCardNumberTransactionRequest {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_payment_methods_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionYearMethodResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction method records for card {masked_card} year {}",
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
                    "fetch yearly transaction METHOD for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl TransactionStatsStatusByCardNumberGrpcClientTrait for TransactionGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_success_bycard(
        &self,
        req: &DomainMonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction SUCCESS status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transaction_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly SUCCESS transaction records for card {masked_card} {month_str} {}",
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
                    "fetch monthly SUCCESS transaction status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_success_bycard(
        &self,
        req: &DomainYearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusSuccess>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction SUCCESS status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transaction_status_success_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusSuccess> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly SUCCESS transaction records for card {masked_card} year {}",
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
                    "fetch yearly SUCCESS transaction status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_month_status_failed_bycard(
        &self,
        req: &DomainMonthStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseMonthStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        let month_str = month_name(req.month);
        info!(
            "fetching monthly transaction FAILED status for card: {masked_card}, {month_str} {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindMonthlyTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        });

        match client
            .find_monthly_transaction_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseMonthStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly FAILED transaction records for card {masked_card} {month_str} {}",
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
                    "fetch monthly FAILED transaction status for card {masked_card} {month_str} {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_status_failed_bycard(
        &self,
        req: &DomainYearStatusTransactionCardNumber,
    ) -> Result<ApiResponse<Vec<TransactionResponseYearStatusFailed>>, AppErrorHttp> {
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "fetching yearly transaction FAILED status for card: {masked_card}, year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearTransactionStatusCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transaction_status_failed_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<TransactionResponseYearStatusFailed> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly FAILED transaction records for card {masked_card} year {}",
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
                    "fetch yearly FAILED transaction status for card {masked_card} year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}
