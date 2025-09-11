use genproto::transaction::{
    ApiResponsePaginationTransaction, ApiResponsePaginationTransactionDeleteAt,
    ApiResponseTransaction, ApiResponseTransactionAll, ApiResponseTransactionDelete,
    ApiResponseTransactionDeleteAt, ApiResponseTransactionMonthAmount,
    ApiResponseTransactionMonthMethod, ApiResponseTransactionMonthStatusFailed,
    ApiResponseTransactionMonthStatusSuccess, ApiResponseTransactionYearAmount,
    ApiResponseTransactionYearMethod, ApiResponseTransactionYearStatusFailed,
    ApiResponseTransactionYearStatusSuccess, ApiResponseTransactions, CreateTransactionRequest,
    FindAllTransactionCardNumberRequest, FindAllTransactionRequest,
    FindByCardNumberTransactionRequest, FindByIdTransactionRequest,
    FindByYearCardNumberTransactionRequest, FindMonthlyTransactionStatus,
    FindMonthlyTransactionStatusCardNumber, FindTransactionByMerchantIdRequest,
    FindYearTransactionStatus, FindYearTransactionStatusCardNumber, UpdateTransactionRequest,
    transaction_service_server::TransactionService,
};

use shared::{
    abstract_trait::transaction::service::{
        command::DynTransactionCommandService,
        query::DynTransactionQueryService,
        stats::{
            amount::DynTransactionStatsAmountService, method::DynTransactionStatsMethodService,
            status::DynTransactionStatsStatusService,
        },
        statsbycard::{
            amount::DynTransactionStatsAmountByCardService,
            method::DynTransactionStatsMethodByCardService,
            status::DynTransactionStatsStatusByCardService,
        },
    },
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
pub struct TransactionStats {
    pub amount: DynTransactionStatsAmountService,
    pub method: DynTransactionStatsMethodService,
    pub status: DynTransactionStatsStatusService,
}

#[derive(Clone)]
pub struct TransactionStatsByCard {
    pub amount: DynTransactionStatsAmountByCardService,
    pub method: DynTransactionStatsMethodByCardService,
    pub status: DynTransactionStatsStatusByCardService,
}

#[derive(Clone)]
pub struct TransactionServiceImpl {
    pub query: DynTransactionQueryService,
    pub command: DynTransactionCommandService,
    pub stats: TransactionStats,
    pub statsbycard: TransactionStatsByCard,
}

impl TransactionServiceImpl {
    pub fn new(
        query: DynTransactionQueryService,
        command: DynTransactionCommandService,
        stats: TransactionStats,
        statsbycard: TransactionStatsByCard,
    ) -> Self {
        Self {
            query,
            command,
            stats,
            statsbycard,
        }
    }
}

#[tonic::async_trait]
impl TransactionService for TransactionServiceImpl {
    async fn find_all_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransaction {
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

    async fn find_all_transaction_by_card_number(
        &self,
        request: Request<FindAllTransactionCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTransaction>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTransactionCardNumber {
            card_number: req.card_number,
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all_by_card_number(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransaction {
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

    async fn find_by_id_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();

        match self.query.find_by_id(req.transaction_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_transaction_status_success(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransaction {
            year: req.year,
            month: req.month,
        };

        match self
            .stats
            .status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_transaction_status_success(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_success(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_transaction_status_failed(
        &self,
        request: Request<FindMonthlyTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransaction {
            year: req.year,
            month: req.month,
        };

        match self.stats.status.get_month_status_failed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_transaction_status_failed(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
        let req = request.into_inner();

        match self.stats.status.get_yearly_status_failed(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransactionCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_transaction_status_success_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusTransactionCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransactionCardNumber {
            card_number: req.card_number,
            year: req.year,
            month: req.month,
        };

        match self
            .statsbycard
            .status
            .get_month_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_transaction_status_failed_by_card_number(
        &self,
        request: Request<FindYearTransactionStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransactionYearStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusTransactionCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .status
            .get_yearly_status_failed(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
        let req = request.into_inner();

        match self.stats.method.get_monthly_method(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_payment_methods(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
        let req = request.into_inner();

        match self.stats.method.get_yearly_method(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_monthly_amounts(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_amounts(
        &self,
        request: Request<FindYearTransactionStatus>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
        let req = request.into_inner();

        match self.stats.amount.get_yearly_amounts(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .method
            .get_monthly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_payment_methods_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self.statsbycard.method.get_yearly_method(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionMonthAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_monthly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_amounts_by_card_number(
        &self,
        request: Request<FindByYearCardNumberTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionYearAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_yearly_amounts(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_transaction_by_merchant_id(
        &self,
        request: Request<FindTransactionByMerchantIdRequest>,
    ) -> Result<Response<ApiResponseTransactions>, Status> {
        let req = request.into_inner();

        match self.query.find_by_merchant_id(req.merchant_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactions {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_active_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransactionDeleteAt {
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

    async fn find_by_trashed_transaction(
        &self,
        request: Request<FindAllTransactionRequest>,
    ) -> Result<Response<ApiResponsePaginationTransactionDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransactionDeleteAt {
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

    async fn create_transaction(
        &self,
        request: Request<CreateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainCreateTransactionRequest {
            card_number: req.card_number,
            amount: req.amount,
            payment_method: req.payment_method,
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        match self.command.create(&req.api_key, &domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_transaction(
        &self,
        request: Request<UpdateTransactionRequest>,
    ) -> Result<Response<ApiResponseTransaction>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_datetime(req.transaction_time)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainUpdateTransactionRequest {
            transaction_id: req.transaction_id,
            card_number: req.card_number,
            amount: req.amount as i64,
            payment_method: req.payment_method,
            merchant_id: Some(req.merchant_id),
            transaction_time: date,
        };

        match self.command.update(&req.api_key, &domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransaction {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trashed(req.transaction_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_transaction(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.transaction_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_transaction_permanent(
        &self,
        request: Request<FindByIdTransactionRequest>,
    ) -> Result<Response<ApiResponseTransactionDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete_permanent(req.transaction_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_transaction(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_transaction_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransactionAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransactionAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
