use async_trait::async_trait;
use genproto::merchant::{
    CreateMerchantRequest, FindAllMerchantApikey, FindAllMerchantRequest,
    FindAllMerchantTransaction, FindByApiKeyRequest, FindByIdMerchantRequest,
    FindByMerchantUserIdRequest, FindYearMerchant, FindYearMerchantByApikey, FindYearMerchantById,
    UpdateMerchantRequest, merchant_service_client::MerchantServiceClient,
};
use shared::{
    abstract_trait::merchant::http::{
        MerchantCommandGrpcClientTrait, MerchantGrpcClientServiceTrait,
        MerchantQueryGrpcClientTrait, MerchantStatsAmountByApiKeyGrpcClientTrait,
        MerchantStatsAmountByMerchantGrpcClientTrait, MerchantStatsAmountGrpcClientTrait,
        MerchantStatsMethodByApiKeyGrpcClientTrait, MerchantStatsMethodByMerchantGrpcClientTrait,
        MerchantStatsMethodGrpcClientTrait, MerchantStatsTotalAmountByApiKeyGrpcClientTrait,
        MerchantStatsTotalAmountByMerchantGrpcClientTrait, MerchantStatsTotalAmountGrpcClientTrait,
        MerchantTransactionGrpcClientTrait,
    },
    domain::{
        requests::merchant::{
            CreateMerchantRequest as DomainCreateMerchantRequest,
            FindAllMerchantTransactions as DomainFindAllMerchantTransactions,
            FindAllMerchantTransactionsByApiKey as DomainFindAllMerchantTransactionsByApiKey,
            FindAllMerchantTransactionsById as DomainFindAllMerchantTransactionsById,
            FindAllMerchants as DomainFindAllMerchants,
            MonthYearAmountApiKey as DomainMonthYearAmountApiKey,
            MonthYearAmountMerchant as DomainMonthYearAmountMerchant,
            MonthYearPaymentMethodApiKey as DomainMonthYearPaymentMethodApiKey,
            MonthYearPaymentMethodMerchant as DomainMonthYearPaymentMethodMerchant,
            MonthYearTotalAmountApiKey as DomainMonthYearTotalAmountApiKey,
            MonthYearTotalAmountMerchant as DomainMonthYearTotalAmountMerchant,
            UpdateMerchantRequest as DomainUpdateMerchantRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, MerchantResponse, MerchantResponseDeleteAt,
            MerchantResponseMonthlyAmount, MerchantResponseMonthlyPaymentMethod,
            MerchantResponseMonthlyTotalAmount, MerchantResponseYearlyAmount,
            MerchantResponseYearlyPaymentMethod, MerchantResponseYearlyTotalAmount,
            MerchantTransactionResponse,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct MerchantGrpcClientService {
    client: Arc<Mutex<MerchantServiceClient<Channel>>>,
}

impl MerchantGrpcClientService {
    pub async fn new(client: Arc<Mutex<MerchantServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MerchantGrpcClientServiceTrait for MerchantGrpcClientService {}

#[async_trait]
impl MerchantQueryGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponse>>, AppErrorHttp> {
        info!(
            "fetching all merchants - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} merchants", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_all merchants failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_active(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active merchants - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_active(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active merchants", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_active merchants failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_trashed(
        &self,
        request: &DomainFindAllMerchants,
    ) -> Result<ApiResponsePagination<Vec<MerchantResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed merchants - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_by_trashed(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed merchants", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_trashed merchants failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_apikey(
        &self,
        api_key: &str,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp> {
        info!("fetching merchant by api_key: *** (masked)");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByApiKeyRequest {
            api_key: api_key.to_string(),
        });

        match client.find_by_api_key(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("merchant with api_key - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found merchant by api_key");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find merchant by api_key failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_merchant_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponse>>, AppErrorHttp> {
        info!("fetching merchants by user_id: {user_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByMerchantUserIdRequest { user_id });

        match client.find_by_merchant_user_id(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} merchants for user_id {user_id}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find merchants by user_id {user_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp> {
        info!("fetching merchant by id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        match client.find_by_id_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("merchant {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found merchant {id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find merchant {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantTransactionGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions(
        &self,
        request: &DomainFindAllMerchantTransactions,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp> {
        info!(
            "fetching all merchant transactions - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantRequest {
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_transaction_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} merchant transactions", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch all merchant transactions failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions_by_api_key(
        &self,
        request: &DomainFindAllMerchantTransactionsByApiKey,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp> {
        info!(
            "fetching merchant transactions by api_key: *** (masked) - page: {}, page_size: {}, search: {:?}",
            request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantApikey {
            api_key: request.api_key.clone(),
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_transaction_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} merchant transactions for api_key", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch merchant transactions by api_key failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transactiions_by_id(
        &self,
        request: &DomainFindAllMerchantTransactionsById,
    ) -> Result<ApiResponsePagination<Vec<MerchantTransactionResponse>>, AppErrorHttp> {
        info!(
            "fetching merchant transactions for merchant_id: {} - page: {}, page_size: {}, search: {:?}",
            request.merchant_id, request.page, request.page_size, request.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllMerchantTransaction {
            merchant_id: request.merchant_id,
            page: request.page,
            page_size: request.page_size,
            search: request.search.clone(),
        });

        match client.find_all_transaction_by_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantTransactionResponse> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} transactions for merchant {}",
                    data.len(),
                    request.merchant_id
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
                    "fetch transactions for merchant {} failed: {status:?}",
                    request.merchant_id
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantCommandGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, request), level = "info")]
    async fn create(
        &self,
        request: &DomainCreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp> {
        info!("creating merchant for user_id: {}", request.user_id);

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(CreateMerchantRequest {
            name: request.name.clone(),
            user_id: request.user_id,
        });

        match client.create_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!(
                        "merchant creation failed - data missing in gRPC response for user_id: {}",
                        request.user_id
                    );
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!(
                    "merchant created successfully for user_id: {}",
                    request.user_id
                );
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!(
                    "create merchant for user_id {} failed: {status:?}",
                    request.user_id
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update(
        &self,
        request: &DomainUpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, AppErrorHttp> {
        let merchant_id = request.merchant_id.ok_or_else(|| {
            AppErrorHttp(AppErrorGrpc::Unhandled(
                "merchant_id is required".to_string(),
            ))
        })?;

        info!("updating merchant id: {merchant_id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(UpdateMerchantRequest {
            merchant_id: merchant_id,
            user_id: request.user_id,
            name: request.name.clone(),
            status: request.status.clone(),
        });

        match client.update_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("update merchant {merchant_id} - data missing in gRPC response",);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!("merchant {merchant_id} updated successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update merchant {merchant_id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, AppErrorHttp> {
        info!("trashing merchant id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        match client.trashed_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("trash merchant {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!("merchant {id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash merchant {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(
        &self,
        id: i32,
    ) -> Result<ApiResponse<MerchantResponseDeleteAt>, AppErrorHttp> {
        info!("restoring merchant id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        match client.restore_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("restore merchant {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Merchant data is missing in gRPC response".into(),
                    ))
                })?;

                info!("merchant {id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore merchant {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting merchant id: {id}");

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindByIdMerchantRequest { merchant_id: id });

        match client.delete_merchant_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("merchant {id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete merchant {id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed merchants");

        let mut client = self.client.lock().await;

        match client.restore_all_merchant(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed merchants restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all merchants failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all merchants");

        let mut client = self.client.lock().await;

        match client.delete_all_merchant_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all merchants permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all merchants permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsAmountGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp> {
        info!("fetching monthly AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_monthly_amount_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp> {
        info!("fetching yearly AMOUNT stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_yearly_amount_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly amount records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly AMOUNT for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp> {
        info!("fetching monthly PAYMENT METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_monthly_payment_methods_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly payment method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly PAYMENT METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_method(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp> {
        info!("fetching yearly PAYMENT METHOD stats for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_yearly_payment_method_merchant(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly payment method records for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly PAYMENT METHOD for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountGrpcClientTrait for MerchantGrpcClientService {
    async fn get_monthly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_monthly_total_amount_merchant(grpc_req).await {
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

    async fn get_yearly_total_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindYearMerchant { year });

        match client.find_yearly_total_amount_merchant(grpc_req).await {
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

#[async_trait]
impl MerchantStatsAmountByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_bymerchant(
        &self,
        req: &DomainMonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client.find_monthly_amount_by_merchants(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly amount records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch monthly AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_bymerchant(
        &self,
        req: &DomainMonthYearAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client.find_yearly_amount_by_merchants(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly amount records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch yearly AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_bymerchant(
        &self,
        req: &DomainMonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp> {
        info!(
            "fetching monthly PAYMENT METHOD for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client
            .find_monthly_payment_method_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly payment method records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch monthly PAYMENT METHOD for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_bymerchant(
        &self,
        req: &DomainMonthYearPaymentMethodMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp> {
        info!(
            "fetching yearly PAYMENT METHOD for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client
            .find_yearly_payment_method_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly payment method records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch yearly PAYMENT METHOD for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByMerchantGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_total_amount_bymerchant(
        &self,
        req: &DomainMonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly TOTAL AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client
            .find_monthly_total_amount_by_merchants(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly total amount records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch monthly TOTAL AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_total_amount_bymerchant(
        &self,
        req: &DomainMonthYearTotalAmountMerchant,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly TOTAL AMOUNT for merchant_id: {}, year: {}",
            req.merchant_id, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantById {
            merchant_id: req.merchant_id,
            year: req.year,
        });

        match client.find_yearly_total_amount_by_merchants(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly total amount records for merchant {} year {}",
                    data.len(),
                    req.merchant_id,
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
                    "fetch yearly TOTAL AMOUNT for merchant {} year {} failed: {status:?}",
                    req.merchant_id, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsAmountByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount_byapikey(
        &self,
        req: &DomainMonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_monthly_amount_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly amount records for api_key year {}",
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
                    "fetch monthly AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount_byapikey(
        &self,
        req: &DomainMonthYearAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_yearly_amount_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly amount records for api_key year {}",
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
                    "fetch yearly AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsMethodByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_method_byapikey(
        &self,
        req: &DomainMonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyPaymentMethod>>, AppErrorHttp> {
        info!(
            "fetching monthly PAYMENT METHOD by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_monthly_payment_method_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly payment method records for api_key year {}",
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
                    "fetch monthly PAYMENT METHOD by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_method_byapikey(
        &self,
        req: &DomainMonthYearPaymentMethodApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyPaymentMethod>>, AppErrorHttp> {
        info!(
            "fetching yearly PAYMENT METHOD by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_yearly_payment_method_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyPaymentMethod> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly payment method records for api_key year {}",
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
                    "fetch yearly PAYMENT METHOD by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl MerchantStatsTotalAmountByApiKeyGrpcClientTrait for MerchantGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_total_amount_byapikey(
        &self,
        req: &DomainMonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseMonthlyTotalAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly TOTAL AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_monthly_total_amount_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseMonthlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly total amount records for api_key year {}",
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
                    "fetch monthly TOTAL AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_total_amount_byapikey(
        &self,
        req: &DomainMonthYearTotalAmountApiKey,
    ) -> Result<ApiResponse<Vec<MerchantResponseYearlyTotalAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly TOTAL AMOUNT by api_key: *** (masked) - year: {}",
            req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearMerchantByApikey {
            api_key: req.api_key.clone(),
            year: req.year,
        });

        match client.find_yearly_total_amount_by_apikey(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<MerchantResponseYearlyTotalAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly total amount records for api_key year {}",
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
                    "fetch yearly TOTAL AMOUNT by api_key for year {} failed: {status:?}",
                    req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}
