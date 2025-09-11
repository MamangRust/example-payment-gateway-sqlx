use genproto::transfer::{
    ApiResponsePaginationTransfer, ApiResponsePaginationTransferDeleteAt, ApiResponseTransfer,
    ApiResponseTransferAll, ApiResponseTransferDelete, ApiResponseTransferDeleteAt,
    ApiResponseTransferMonthAmount, ApiResponseTransferMonthStatusFailed,
    ApiResponseTransferMonthStatusSuccess, ApiResponseTransferYearAmount,
    ApiResponseTransferYearStatusFailed, ApiResponseTransferYearStatusSuccess,
    ApiResponseTransfers, CreateTransferRequest, FindAllTransferRequest,
    FindByCardNumberTransferRequest, FindByIdTransferRequest, FindMonthlyTransferStatus,
    FindMonthlyTransferStatusCardNumber, FindTransferByTransferFromRequest,
    FindTransferByTransferToRequest, FindYearTransferStatus, FindYearTransferStatusCardNumber,
    UpdateTransferRequest, transfer_service_server::TransferService,
};
use shared::{
    abstract_trait::transfer::service::{
        command::DynTransferCommandService,
        query::DynTransferQueryService,
        stats::{amount::DynTransferStatsAmountService, status::DynTransferStatsStatusService},
        statsbycard::{
            amount::DynTransferStatsAmountByCardService,
            status::DynTransferStatsStatusByCardService,
        },
    },
    domain::requests::transfer::{
        CreateTransferRequest as DomainCreateTransferRequest, FindAllTransfers,
        MonthStatusTransfer, MonthStatusTransferCardNumber, MonthYearCardNumber,
        UpdateTransferRequest as DomainUpdateTransferRequest, YearStatusTransferCardNumber,
    },
    errors::AppErrorGrpc,
};

use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct TransferStats {
    pub amount: DynTransferStatsAmountService,
    pub status: DynTransferStatsStatusService,
}

#[derive(Clone)]
pub struct TransferStatsByCard {
    pub amount: DynTransferStatsAmountByCardService,
    pub status: DynTransferStatsStatusByCardService,
}

#[derive(Clone)]
pub struct TransferServiceImpl {
    pub query: DynTransferQueryService,
    pub command: DynTransferCommandService,
    pub stats: TransferStats,
    pub statsbycard: TransferStatsByCard,
}

impl TransferServiceImpl {
    pub fn new(
        query: DynTransferQueryService,
        command: DynTransferCommandService,
        stats: TransferStats,
        statsbycard: TransferStatsByCard,
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
impl TransferService for TransferServiceImpl {
    #[instrument(skip(self, request), level = "info", err)]
    async fn find_all_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransfer>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllTransfers {
            page: req.page,
            page_size: req.page_size,
            search: req.search.clone(),
        };

        info!("📦 Fetching all transfers...");
        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched {} transfers",
                    api_response.data.len()
                );
                let grpc_response = ApiResponsePaginationTransfer {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_by_id_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        let req = request.into_inner();

        info!("🔍 Fetching transfer by ID: {}", req.transfer_id);
        match self.query.find_by_id(req.transfer_id).await {
            Ok(api_response) => {
                info!("✅ Transfer found.");
                let grpc_response = ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Transfer not found: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transfer_status_success(
        &self,
        request: Request<FindMonthlyTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransfer {
            year: req.year,
            month: req.month,
        };

        info!(
            "📊 Fetching monthly successful transfer status for {}/{}",
            req.month, req.year
        );
        match self
            .stats
            .status
            .get_month_status_success(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly success records",
                    api_response.data.len()
                );
                let grpc_response = ApiResponseTransferMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly success transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transfer_status_success(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearStatusSuccess>, Status> {
        let req = request.into_inner();

        info!(
            "📊 Fetching yearly successful transfer status for {}",
            req.year
        );
        match self.stats.status.get_yearly_status_success(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly success records",
                    api_response.data.len()
                );
                let grpc_response = ApiResponseTransferYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly success transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transfer_status_failed(
        &self,
        request: Request<FindMonthlyTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransfer {
            year: req.year,
            month: req.month,
        };

        info!(
            "📊 Fetching monthly failed transfer status for {}/{}",
            req.month, req.year
        );
        match self.stats.status.get_month_status_failed(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly failed records",
                    api_response.data.len()
                );
                let grpc_response = ApiResponseTransferMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly failed transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transfer_status_failed(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearStatusFailed>, Status> {
        let req = request.into_inner();

        info!("📊 Fetching yearly failed transfer status for {}", req.year);
        match self.stats.status.get_yearly_status_failed(req.year).await {
            Ok(api_response) => {
                info!("✅ Found {} yearly failed records", api_response.data.len());
                let grpc_response = ApiResponseTransferYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly failed transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transfer_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransferCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        };

        info!("📊 Fetching monthly SUCCESS transfer status by card...");
        match self
            .statsbycard
            .status
            .get_month_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly success records for card {}",
                    api_response.data.len(),
                    domain_req.card_number
                );
                let grpc_response = ApiResponseTransferMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly success transfers by card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transfer_status_success_by_card_number(
        &self,
        request: Request<FindYearTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferYearStatusSuccess>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusTransferCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        info!("📊 Fetching yearly SUCCESS transfer status by card...");
        match self
            .statsbycard
            .status
            .get_yearly_status_success_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly success records for card {}",
                    api_response.data.len(),
                    domain_req.card_number
                );
                let grpc_response = ApiResponseTransferYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly success transfers by card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transfer_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = MonthStatusTransferCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
            month: req.month,
        };

        info!("📊 Fetching monthly FAILED transfer status by card...");
        match self
            .statsbycard
            .status
            .get_month_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly failed records for card {}",
                    api_response.data.len(),
                    domain_req.card_number
                );
                let grpc_response = ApiResponseTransferMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly failed transfers by card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transfer_status_failed_by_card_number(
        &self,
        request: Request<FindYearTransferStatusCardNumber>,
    ) -> Result<Response<ApiResponseTransferYearStatusFailed>, Status> {
        let req = request.into_inner();
        let domain_req = YearStatusTransferCardNumber {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        info!("📊 Fetching yearly FAILED transfer status by card...");
        match self
            .statsbycard
            .status
            .get_yearly_status_failed_by_card(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!(
                    "✅ Found {} yearly failed records for card {}",
                    api_response.data.len(),
                    domain_req.card_number
                );
                let grpc_response = ApiResponseTransferYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly failed transfers by card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_monthly_transfer_amounts(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
        let req = request.into_inner();

        info!("📊 Fetching monthly transfer amounts for year {}", req.year);
        match self.stats.amount.get_monthly_amounts(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Found {} monthly amount records",
                    api_response.data.len()
                );
                let grpc_response = ApiResponseTransferMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly transfer amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info", err)]
    async fn find_yearly_transfer_amounts(
        &self,
        request: Request<FindYearTransferStatus>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
        let req = request.into_inner();

        info!("📊 Fetching yearly transfer amounts for year {}", req.year);
        match self.stats.amount.get_yearly_amounts(req.year).await {
            Ok(api_response) => {
                info!("✅ Found {} yearly amount records", api_response.data.len());
                let grpc_response = ApiResponseTransferYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly transfer amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_transfer_amounts_by_sender_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly transfer amounts by sender card number: {}",
            req.card_number
        );

        let domain_req = MonthYearCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_monthly_amounts_by_sender(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly amounts by sender card: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_transfer_amounts_by_sender_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly transfer amounts by sender card number: {}",
            req.card_number
        );

        let domain_req = MonthYearCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_yearly_amounts_by_sender(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly amounts by sender card: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_transfer_amounts_by_receiver_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferMonthAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly transfer amounts by receiver card number: {}",
            req.card_number
        );

        let domain_req = MonthYearCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_monthly_amounts_by_receiver(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly amounts by receiver card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_transfer_amounts_by_receiver_card_number(
        &self,
        request: Request<FindByCardNumberTransferRequest>,
    ) -> Result<Response<ApiResponseTransferYearAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly transfer amounts by receiver card number: {}",
            req.card_number
        );

        let domain_req = MonthYearCardNumber {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .amount
            .get_yearly_amounts_by_receiver(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly amounts by receiver card: {:?}",
                    e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_transfer_by_transfer_from(
        &self,
        request: Request<FindTransferByTransferFromRequest>,
    ) -> Result<Response<ApiResponseTransfers>, Status> {
        let req = request.into_inner();
        info!(
            "🔎 Finding transfers by transfer_from: {}",
            req.transfer_from
        );

        match self.query.find_by_transfer_from(&req.transfer_from).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransfers {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to find transfers by transfer_from: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_transfer_by_transfer_to(
        &self,
        request: Request<FindTransferByTransferToRequest>,
    ) -> Result<Response<ApiResponseTransfers>, Status> {
        let req = request.into_inner();
        info!("🔎 Finding transfers by transfer_to: {}", req.transfer_to);

        match self.query.find_by_transfer_to(&req.transfer_to).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransfers {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to find transfers by transfer_to: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_active_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransferDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "📦 Finding active transfers, page: {}, page_size: {}",
            req.page, req.page_size
        );

        let domain_req = FindAllTransfers {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransferDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch active transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_trashed_transfer(
        &self,
        request: Request<FindAllTransferRequest>,
    ) -> Result<Response<ApiResponsePaginationTransferDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🗑️ Finding trashed transfers, page: {}, page_size: {}",
            req.page, req.page_size
        );

        let domain_req = FindAllTransfers {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_by_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTransferDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn create_transfer(
        &self,
        request: Request<CreateTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        let req = request.into_inner();
        info!(
            "➕ Creating transfer: from {} to {}, amount {}",
            req.transfer_from, req.transfer_to, req.transfer_amount
        );

        let domain_req = DomainCreateTransferRequest {
            transfer_from: req.transfer_from,
            transfer_to: req.transfer_to,
            transfer_amount: req.transfer_amount as i64,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to create transfer: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update_transfer(
        &self,
        request: Request<UpdateTransferRequest>,
    ) -> Result<Response<ApiResponseTransfer>, Status> {
        let req = request.into_inner();
        info!(
            "♻️ Updating transfer: id {}, from {} to {}, amount {}",
            req.transfer_id, req.transfer_from, req.transfer_to, req.transfer_amount
        );

        let domain_req = DomainUpdateTransferRequest {
            transfer_id: req.transfer_id,
            transfer_from: req.transfer_from,
            transfer_to: req.transfer_to,
            transfer_amount: req.transfer_amount as i64,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransfer {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to update transfer: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn trashed_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDeleteAt>, Status> {
        let req = request.into_inner();
        info!("🗑️ Trashing transfer with id: {}", req.transfer_id);

        match self.command.trashed(req.transfer_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to trash transfer: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn restore_transfer(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDeleteAt>, Status> {
        let req = request.into_inner();
        info!("♻️ Restoring transfer with id: {}", req.transfer_id);

        match self.command.restore(req.transfer_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore transfer: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn delete_transfer_permanent(
        &self,
        request: Request<FindByIdTransferRequest>,
    ) -> Result<Response<ApiResponseTransferDelete>, Status> {
        let req = request.into_inner();
        info!(
            "🗑️ Permanently deleting transfer with id: {}",
            req.transfer_id
        );

        match self.command.delete_permanent(req.transfer_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete transfer: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all_transfer(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransferAll>, Status> {
        info!("♻️ Restoring all trashed transfers");

        match self.command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore all transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_transfer_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTransferAll>, Status> {
        info!("🗑️ Permanently deleting all transfers");

        match self.command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTransferAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all transfers: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
