use genproto::transaction::{
    ApiResponsePaginationTransaction, ApiResponsePaginationTransactionDeleteAt,
    ApiResponseTransaction, ApiResponseTransactionAll, ApiResponseTransactionDelete,
    ApiResponseTransactionDeleteAt, ApiResponseTransactionMonthAmount,
    ApiResponseTransactionMonthMethod, ApiResponseTransactionMonthStatusFailed,
    ApiResponseTransactionMonthStatusSuccess, ApiResponseTransactionYearAmount,
    ApiResponseTransactionYearMethod, ApiResponseTransactionYearStatusFailed,
    ApiResponseTransactionYearStatusSuccess, ApiResponseTransactions, CreateTransactionRequest,
    FindAllTransactionCardNumberRequest, FindAllTransactionRequest, FindByIdTransactionRequest,
    FindByYearCardNumberTransactionRequest, FindMonthlyTransactionStatus,
    FindMonthlyTransactionStatusCardNumber, FindTransactionByMerchantIdRequest,
    FindYearTransactionStatus, FindYearTransactionStatusCardNumber, UpdateTransactionRequest,
    transaction_service_server::TransactionService,
};
use std::sync::Arc;

use crate::di::DependenciesInject;
use shared::{
    domain::requests::transaction::{
        CreateTransactionRequest as DomainCreateTransactionRequest, FindAllTransactionCardNumber,
        FindAllTransactions, MonthStatusTransaction, MonthStatusTransactionCardNumber,
        MonthYearPaymentMethod, UpdateTransactionRequest as DomainUpdateTransactionRequest,
        YearStatusTransactionCardNumber,
    },
    errors::AppErrorGrpc,
    utils::timestamp_to_naive_datetime,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct TransactionServiceImpl {
    pub di: Arc<DependenciesInject>,
}

impl TransactionServiceImpl {
    pub fn new(di: Arc<DependenciesInject>) -> Self {
        Self { di }
    }
}

#[tonic::async_trait]
impl TransactionService for TransactionServiceImpl {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        let req = request.into_inner();
        info!(
            "📄 Fetching all transactions | page: {}, page_size: {}",
            req.page, req.page_size
        );

        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.transaction_query.find_all(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} transactions", api_response.data.len());
                Ok(Response::new(ApiResponsePaginationTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_transaction_by_card_number(
        &self,
        request: Request<FindAllTransactionCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        let req = request.into_inner();
        info!(
            "📄 Fetching transactions by card | card_number: {}, page: {}, page_size: {}",
            req.card_number, req.page, req.page_size
        );

        let domain_req = FindAllTransactionCardNumber {
            card_number: req.card_number.clone(),
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self
            .di
            .transaction_query
            .find_all_by_card_number(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} transactions for card {}",
                    api_response.data.len(),
                    req.card_number
                );
                Ok(Response::new(ApiResponsePaginationTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch transactions for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_id_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();
        info!("🔍 Fetching transaction by id | id: {}", req.transaction_id);

        match self
            .di
            .transaction_query
            .find_by_id(req.transaction_id)
            .await
        {
            Ok(api_response) => {
                info!("✅ Transaction {} found", req.transaction_id);
                Ok(Response::new(ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch transaction {}: {:?}",
                    req.transaction_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_transaction_status_success(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly success status | year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthStatusTransaction {
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .transaction_stats_status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} monthly success stats", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly success stats: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_transaction_status_success(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly success status | year: {}", req.year);

        match self
            .di
            .transaction_stats_status
            .get_yearly_status_success(req.year)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} yearly success stats", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly success stats: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_transaction_status_failed(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly failed status | year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthStatusTransaction {
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .transaction_stats_status
            .get_month_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} monthly failed stats", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly failed stats: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_transaction_status_failed(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly failed status | year: {}", req.year);

        match self
            .di
            .transaction_stats_status
            .get_yearly_status_failed(req.year)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} yearly failed stats", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly failed stats: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly success transactions by card | card: {}, year: {}, month: {}",
            req.card_number, req.year, req.month
        );

        let domain_req = MonthStatusTransactionCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .transaction_stats_status_by_card
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} success transactions", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly success stats for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly success transactions by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearStatusTransactionCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_status_by_card
            .get_yearly_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly success transactions",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly success stats for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly failed transactions by card | card: {}, year: {}, month: {}",
            req.card_number, req.year, req.month
        );

        let domain_req = MonthStatusTransactionCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        };

        match self
            .di
            .transaction_stats_status_by_card
            .get_month_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} failed transactions", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly failed stats for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly failed transactions by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearStatusTransactionCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_status_by_card
            .get_yearly_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly failed transactions",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly failed stats for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching monthly payment methods | year: {}", req.year);

        match self
            .di
            .transaction_stats_method
            .get_monthly_method(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly payment methods",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly payment methods: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly payment methods | year: {}", req.year);

        match self
            .di
            .transaction_stats_method
            .get_yearly_method(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly payment methods",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly payment methods: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching monthly amounts | year: {}", req.year);

        match self
            .di
            .transaction_stats_amount
            .get_monthly_amounts(req.year)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly amount records",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly amounts for year {}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly amounts | year: {}", req.year);

        match self
            .di
            .transaction_stats_amount
            .get_yearly_amounts(req.year)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} yearly amount records", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly amounts for year {}: {:?}",
                    req.year, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly payment methods by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_method_by_card
            .get_monthly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly payment methods",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly methods for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly payment methods by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_method_by_card
            .get_yearly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly payment methods",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly methods for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly amounts by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_amount_by_card
            .get_monthly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly amount records",
                    api_response.data.len()
                );
                Ok(Response::new(ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly amounts for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly amounts by card | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self
            .di
            .transaction_stats_amount_by_card
            .get_yearly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} yearly amount records", api_response.data.len());
                Ok(Response::new(ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly amounts for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_transaction_by_merchant_id(
        &self,
        request: Request<FindTransactionByMerchantIdRequest>,
    ) -> Result<Response<ApiResponseTransactions>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Finding transactions for merchant_id={}",
            req.merchant_id
        );

        match self
            .di
            .transaction_query
            .find_by_merchant_id(req.merchant_id)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} transactions", api_response.data.len());
                let grpc_response = ApiResponseTransactions {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to find transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_active_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Fetching active transactions page={} page_size={}",
            req.page, req.page_size
        );

        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        match self.di.transaction_query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} active transactions", api_response.data.len());
                let grpc_response = ApiResponsePaginationTransactionDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch active transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_trashed_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🔍 Fetching trashed transactions page={} page_size={}",
            req.page, req.page_size
        );

        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        match self.di.transaction_query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} trashed transactions", api_response.data.len());
                let grpc_response = ApiResponsePaginationTransactionDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn create_transaction(
        &self,
        request: Request<CreateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();
        info!(
            "📝 Creating transaction for card_number={}",
            req.card_number
        );

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("transaction_time invalid"))?;

        let domain_req = DomainCreateTransactionRequest {
            card_number: req.card_number.clone(),
            amount: req.amount,
            payment_method: req.payment_method.clone(),
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        match self
            .di
            .transaction_command
            .create(&req.api_key, &domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Transaction created id={}", api_response.data.id);
                let grpc_response = ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to create transaction: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update_transaction(
        &self,
        request: Request<UpdateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();
        info!("✏️ Updating transaction id={}", req.transaction_id);

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("transaction_time invalid"))?;

        let domain_req = DomainUpdateTransactionRequest {
            transaction_id: Some(req.transaction_id),
            card_number: req.card_number.clone(),
            amount: req.amount as i64,
            payment_method: req.payment_method.clone(),
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        match self
            .di
            .transaction_command
            .update(&req.api_key, &domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Transaction updated id={}", api_response.data.id);
                let grpc_response = ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to update transaction: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn trashed_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        info!("🗑 Trashing transaction id={}", req.transaction_id);

        match self
            .di
            .transaction_command
            .trashed(req.transaction_id)
            .await
        {
            Ok(api_response) => {
                info!("✅ Transaction moved to trash id={}", req.transaction_id);
                let grpc_response = ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to trash transaction: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn restore_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        info!("♻️ Restoring transaction id={}", req.transaction_id);

        match self
            .di
            .transaction_command
            .restore(req.transaction_id)
            .await
        {
            Ok(api_response) => {
                info!("✅ Transaction restored id={}", req.transaction_id);
                let grpc_response = ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore transaction: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn delete_transaction_permanent(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDelete>, Status> {
        let req = request.into_inner();
        info!(
            "💀 Permanently deleting transaction id={}",
            req.transaction_id
        );

        match self
            .di
            .transaction_command
            .delete_permanent(req.transaction_id)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Transaction permanently deleted id={}",
                    req.transaction_id
                );
                let grpc_response = ApiResponseTransactionDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete transaction: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all_transaction(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        info!("♻️ Restoring ALL trashed transactions");

        match self.di.transaction_command.restore_all().await {
            Ok(api_response) => {
                info!("✅ All trashed transactions restored");
                let grpc_response = ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore all transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_transaction_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        info!("💀 Permanently deleting ALL trashed transactions");

        match self.di.transaction_command.delete_all().await {
            Ok(api_response) => {
                info!("✅ All trashed transactions permanently deleted");
                let grpc_response = ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all transactions: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
