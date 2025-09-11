use std::sync::Arc;

use async_trait::async_trait;
use genproto::card::{
    CreateCardRequest, FindAllCardRequest, FindByCardNumberRequest, FindByIdCardRequest,
    FindByUserIdCardRequest, FindYearAmount, FindYearAmountCardNumber, FindYearBalance,
    FindYearBalanceCardNumber, UpdateCardRequest, card_service_client::CardServiceClient,
};
use shared::{
    abstract_trait::card::http::{
        command::CardCommandGrpcClientTrait,
        dashboard::CardDashboardGrpcClientTrait,
        query::CardQueryGrpcClientTrait,
        stats::{
            balance::CardStatsBalanceGrpcClientTrait, topup::CardStatsTopupGrpcClientTrait,
            transaction::CardStatsTransactionGrpcClientTrait,
            transfer::CardStatsTransferGrpcClientTrait, withdraw::CardStatsWithdrawGrpcClientTrait,
        },
        statsbycard::{
            balance::CardStatsBalanceByCardGrpcClientTrait,
            topup::CardStatsTopupByCardGrpcClientTrait,
            transaction::CardStatsTransactionByCardGrpcClientTrait,
            transfer::CardStatsTransferByCardGrpcClientTrait,
            withdraw::CardStatsWithdrawByCardGrpcClientTrait,
        },
    },
    domain::{
        requests::card::{
            CreateCardRequest as DomainCreateCardRequest, FindAllCards as DomainFindAllCardRequest,
            MonthYearCardNumberCard as DomainMonthYearCardNumberCard,
            UpdateCardRequest as DomainUpdateCardRequest,
        },
        responses::{
            ApiResponse, ApiResponsePagination, CardResponse, CardResponseDeleteAt,
            CardResponseMonthAmount, CardResponseMonthBalance, CardResponseYearAmount,
            CardResponseYearlyBalance, DashboardCard, DashboardCardCardNumber,
        },
    },
    errors::{AppErrorGrpc, AppErrorHttp},
    utils::naive_date_to_timestamp,
};
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info, instrument};

#[async_trait]
pub trait CardGrpcClientServiceTrait:
    CardCommandGrpcClientTrait
    + CardQueryGrpcClientTrait
    + CardStatsBalanceGrpcClientTrait
    + CardStatsTopupGrpcClientTrait
    + CardStatsTransactionGrpcClientTrait
    + CardStatsTransferGrpcClientTrait
    + CardStatsWithdrawGrpcClientTrait
    + CardStatsBalanceByCardGrpcClientTrait
    + CardStatsTopupByCardGrpcClientTrait
    + CardStatsTransactionByCardGrpcClientTrait
    + CardStatsTransferByCardGrpcClientTrait
    + CardStatsWithdrawByCardGrpcClientTrait
    + CardStatsWithdrawByCardGrpcClientTrait
{
}

#[derive(Debug)]
pub struct CardGrpcClientService {
    client: Arc<Mutex<CardServiceClient<Channel>>>,
}

impl CardGrpcClientService {
    pub async fn new(client: Arc<Mutex<CardServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CardDashboardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip_all)]
    async fn get_dashboard(&self) -> Result<ApiResponse<DashboardCard>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        match client.dashboard_card(()).await {
            Ok(response) => {
                let inner = response.into_inner();

                let dashboard_data = inner.data.ok_or_else(|| {
                    error!("Dashboard Card data is missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Dashboard Card data is missing in gRPC response".into(),
                    ))
                })?;

                let domain_dashboard: DashboardCard = dashboard_data.into();

                Ok(ApiResponse {
                    status: inner.status,
                    message: inner.message,
                    data: domain_dashboard,
                })
            }
            Err(status) => {
                error!("gRPC error: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip_all, fields(card_number = card_number))]
    async fn get_dashboard_bycard(
        &self,
        card_number: &str,
    ) -> Result<ApiResponse<DashboardCardCardNumber>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let request = Request::new(FindByCardNumberRequest {
            card_number: card_number.to_string(),
        });

        match client.dashboard_card_number(request).await {
            Ok(response) => {
                let inner = response.into_inner();

                let dashboard_data = inner.data.ok_or_else(|| {
                    error!("card {card_number} - missing data in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Dashboard Card data is missing in gRPC response".into(),
                    ))
                })?;

                let domain_dashboard: DashboardCardCardNumber = dashboard_data.into();

                Ok(ApiResponse {
                    status: inner.status,
                    message: inner.message,
                    data: domain_dashboard,
                })
            }
            Err(status) => {
                error!("card {card_number} - gRPC failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardQueryGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn find_all(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponse>>, AppErrorHttp> {
        info!(
            "fetching cards - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllCardRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_all_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponse> = inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} cards", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_all failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_active(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching active cards - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllCardRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_active_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} active cards", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_active failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn find_trashed(
        &self,
        req: &DomainFindAllCardRequest,
    ) -> Result<ApiResponsePagination<Vec<CardResponseDeleteAt>>, AppErrorHttp> {
        info!(
            "fetching trashed cards - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let mut client = self.client.lock().await;

        let grpc_req = Request::new(FindAllCardRequest {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        });

        match client.find_by_trashed_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseDeleteAt> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} trashed cards", data.len());
                Ok(ApiResponsePagination {
                    data,
                    pagination: inner.pagination.unwrap_or_default().into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("find_trashed failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_id(&self, id: i32) -> Result<ApiResponse<CardResponse>, AppErrorHttp> {
        info!("fetching card by id: {id}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        match client.find_by_id_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - missing data in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found card {id}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("card {id} - gRPC failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp> {
        info!("fetching card by user_id: {user_id}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByUserIdCardRequest { user_id });

        match client.find_by_user_id_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("user {user_id} - missing card data in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found card for user {user_id}");
                Ok(ApiResponse {
                    message: inner.message,
                    status: inner.status,
                    data: data.into(),
                })
            }
            Err(status) => {
                error!("user {user_id} - gRPC failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn find_by_card_number(
        &self,
        card_number: String,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp> {
        info!("fetching card by card_number: {card_number}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByCardNumberRequest {
            card_number: card_number.clone(),
        });

        match client.find_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {card_number} - missing data in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("found card for number: {card_number}");
                Ok(ApiResponse {
                    data: data.into(),
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("card {card_number} - gRPC failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardCommandGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn create(
        &self,
        req: &DomainCreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp> {
        info!("creating card for user_id: {}", req.user_id);

        let mut client = self.client.lock().await;
        let date = naive_date_to_timestamp(req.expire_date);

        let grpc_req = Request::new(CreateCardRequest {
            user_id: req.user_id,
            card_type: req.card_type.clone(),
            expire_date: Some(date),
            cvv: req.cvv.clone(),
            card_provider: req.card_provider.clone(),
        });

        match client.create_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("user {} - card data missing in gRPC response", req.user_id);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("card created successfully for user {}", req.user_id);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("create card failed for user {}: {status:?}", req.user_id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn update(
        &self,
        req: &DomainUpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, AppErrorHttp> {
        info!("updating card id: {}", req.card_id);

        let mut client = self.client.lock().await;
        let date = naive_date_to_timestamp(req.expire_date);

        let grpc_req = Request::new(UpdateCardRequest {
            card_id: req.card_id,
            user_id: req.user_id,
            card_type: req.card_type.clone(),
            expire_date: Some(date),
            cvv: req.cvv.clone(),
            card_provider: req.card_provider.clone(),
        });

        match client.update_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {} - data missing in gRPC response", req.card_id);
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("card {} updated successfully", req.card_id);
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("update card {} failed: {status:?}", req.card_id);
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, AppErrorHttp> {
        info!("trashing card id: {id}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        match client.trashed_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("card {id} trashed successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("trash card {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, AppErrorHttp> {
        info!("restoring card id: {id}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        match client.restore_card(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data = inner.data.ok_or_else(|| {
                    error!("card {id} - data missing in gRPC response");
                    AppErrorHttp(AppErrorGrpc::Unhandled(
                        "Card data is missing in gRPC response".into(),
                    ))
                })?;

                info!("card {id} restored successfully");
                Ok(ApiResponse {
                    data: data.into(),
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore card {id} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting card id: {id}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindByIdCardRequest { card_id: id });

        match client.delete_card_permanent(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("card {id} permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete card {id} permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("restoring all trashed cards");

        let mut client = self.client.lock().await;

        match client.restore_all_card(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all trashed cards restored successfully");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("restore all cards failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all(&self) -> Result<ApiResponse<bool>, AppErrorHttp> {
        info!("permanently deleting all cards");

        let mut client = self.client.lock().await;

        match client.delete_all_card_permanent(()).await {
            Ok(response) => {
                let inner = response.into_inner();
                info!("all cards permanently deleted");
                Ok(ApiResponse {
                    data: true,
                    status: inner.status,
                    message: inner.message,
                })
            }
            Err(status) => {
                error!("delete all cards permanently failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsBalanceGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, AppErrorHttp> {
        info!("fetching monthly balance for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearBalance { year });

        match client.find_monthly_balance(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} monthly balances for year {year}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly balance for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_balance(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, AppErrorHttp> {
        info!("fetching yearly balance for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearBalance { year });

        match client.find_yearly_balance(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearlyBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                info!("fetched {} yearly balances for year {year}", data.len());
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly balance for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTopupGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!("fetching monthly topup amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_monthly_topup_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly topup amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!("fetching yearly topup amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_yearly_topup_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly topup amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTransactionGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!("fetching monthly TRANSACTION amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_monthly_transaction_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly TRANSACTION amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!("fetching yearly TRANSACTION amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_yearly_transaction_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly TRANSACTION amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTransferGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!("fetching monthly TRANSFER amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_monthly_transfer_sender_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transfer amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly TRANSFER amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!("fetching yearly TRANSFER amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_yearly_transfer_sender_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transfer amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly TRANSFER amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsWithdrawGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self), level = "info")]
    async fn get_monthly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!("fetching monthly WITHDRAW amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_monthly_withdraw_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly withdraw amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch monthly WITHDRAW amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn get_yearly_amount(
        &self,
        year: i32,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!("fetching yearly WITHDRAW amount for year: {year}");

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmount { year });

        match client.find_yearly_withdraw_amount(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly withdraw amounts for year {year}",
                    data.len()
                );
                Ok(ApiResponse {
                    data,
                    message: inner.message,
                    status: inner.status,
                })
            }
            Err(status) => {
                error!("fetch yearly WITHDRAW amount for year {year} failed: {status:?}");
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsBalanceByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_balance(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthBalance>>, AppErrorHttp> {
        info!(
            "fetching monthly BALANCE for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearBalanceCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_monthly_balance_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly balances for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch monthly BALANCE for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_balance(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearlyBalance>>, AppErrorHttp> {
        info!(
            "fetching yearly BALANCE for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearBalanceCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client.find_yearly_balance_by_card_number(grpc_req).await {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearlyBalance> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly balances for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch yearly BALANCE for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTopupByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly TOPUP amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_topup_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly topup amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch monthly TOPUP for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly TOPUP amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_topup_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly topup amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch yearly TOPUP for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTransactionByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly TRANSACTION amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_transaction_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transaction amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch monthly TRANSACTION for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly TRANSACTION amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transaction_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transaction amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch yearly TRANSACTION for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsTransferByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly TRANSFER amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_transfer_sender_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly transfer amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch monthly TRANSFER for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly TRANSFER amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_transfer_sender_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly transfer amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch yearly TRANSFER for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardStatsWithdrawByCardGrpcClientTrait for CardGrpcClientService {
    #[instrument(skip(self, req), level = "info")]
    async fn get_monthly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseMonthAmount>>, AppErrorHttp> {
        info!(
            "fetching monthly WITHDRAW amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_monthly_withdraw_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseMonthAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} monthly withdraw amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch monthly WITHDRAW for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }

    #[instrument(skip(self, req), level = "info")]
    async fn get_yearly_amount(
        &self,
        req: &DomainMonthYearCardNumberCard,
    ) -> Result<ApiResponse<Vec<CardResponseYearAmount>>, AppErrorHttp> {
        info!(
            "fetching yearly WITHDRAW amount for card: {}, year: {}",
            req.card_number, req.year
        );

        let mut client = self.client.lock().await;
        let grpc_req = Request::new(FindYearAmountCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        });

        match client
            .find_yearly_withdraw_amount_by_card_number(grpc_req)
            .await
        {
            Ok(response) => {
                let inner = response.into_inner();
                let data: Vec<CardResponseYearAmount> =
                    inner.data.into_iter().map(Into::into).collect();

                info!(
                    "fetched {} yearly withdraw amounts for card {} year {}",
                    data.len(),
                    req.card_number,
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
                    "fetch yearly WITHDRAW for card {} year {} failed: {status:?}",
                    req.card_number, req.year
                );
                Err(AppErrorHttp(AppErrorGrpc::from(status)))
            }
        }
    }
}

#[async_trait]
impl CardGrpcClientServiceTrait for CardGrpcClientService {}
