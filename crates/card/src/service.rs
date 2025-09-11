use genproto::card::{
    ApiResponseCard, ApiResponseCardAll, ApiResponseCardDelete, ApiResponseCardDeleteAt,
    ApiResponseDashboardCard, ApiResponseDashboardCardNumber, ApiResponseMonthlyAmount,
    ApiResponseMonthlyBalance, ApiResponsePaginationCard, ApiResponsePaginationCardDeleteAt,
    ApiResponseYearlyAmount, ApiResponseYearlyBalance, CreateCardRequest, FindAllCardRequest,
    FindByCardNumberRequest, FindByIdCardRequest, FindByUserIdCardRequest, FindYearAmount,
    FindYearAmountCardNumber, FindYearBalance, FindYearBalanceCardNumber, UpdateCardRequest,
    card_service_server::CardService,
};
use shared::{
    abstract_trait::card::service::{
        command::DynCardCommandService,
        dashboard::DynCardDashboardService,
        query::DynCardQueryService,
        stats::{
            balance::DynCardStatsBalanceService, topup::DynCardStatsTopupService,
            transaction::DynCardStatsTransactionService, transfer::DynCardStatsTransferService,
            withdraw::DynCardStatsWithdrawService,
        },
        statsbycard::{
            balance::DynCardStatsBalanceByCardService, topup::DynCardStatsTopupByCardService,
            transaction::DynCardStatsTransactionByCardService,
            transfer::DynCardStatsTransferByCardService,
            withdraw::DynCardStatsWithdrawByCardService,
        },
    },
    domain::requests::card::{
        CreateCardRequest as DomainCreateCardRequest, FindAllCards, MonthYearCardNumberCard,
        UpdateCardRequest as DomainUpdateCardRequest,
    },
    errors::AppErrorGrpc,
    utils::timestamp_to_naive_date,
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct CardStats {
    pub balance: DynCardStatsBalanceService,
    pub topup: DynCardStatsTopupService,
    pub transaction: DynCardStatsTransactionService,
    pub transfer: DynCardStatsTransferService,
    pub withdraw: DynCardStatsWithdrawService,
}

#[derive(Clone)]
pub struct CardStatsByCard {
    pub balance: DynCardStatsBalanceByCardService,
    pub topup: DynCardStatsTopupByCardService,
    pub transaction: DynCardStatsTransactionByCardService,
    pub transfer: DynCardStatsTransferByCardService,
    pub withdraw: DynCardStatsWithdrawByCardService,
}

#[derive(Clone)]
pub struct CardServiceImpl {
    pub query: DynCardQueryService,
    pub command: DynCardCommandService,
    pub dashboard: DynCardDashboardService,
    pub stats: CardStats,
    pub statsbycard: CardStatsByCard,
}

impl CardServiceImpl {
    pub fn new(
        query: DynCardQueryService,
        command: DynCardCommandService,
        dashboard: DynCardDashboardService,
        stats: CardStats,
        statsbycard: CardStatsByCard,
    ) -> Self {
        Self {
            query,
            command,
            dashboard,
            stats,
            statsbycard,
        }
    }
}


#[tonic::async_trait]
impl CardService for CardServiceImpl {
    #[instrument(skip_all, fields(
        page = request.get_ref().page, 
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_all_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCard>, Status> {
        let req = request.into_inner();

        info!("📥 Received find_all_card request");

        let domain_req = FindAllCards {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let data: Vec<genproto::card::CardResponse> =
                    api_response.data.into_iter().map(Into::into).collect();
                let total_items = api_response.pagination.total_items;

                info!(
                    "✅ Successfully retrieved {} cards 📊 Total items: {total_items}",
                    data.clone().len(),
                );

                let reply = ApiResponsePaginationCard {
                    status:  api_response.status,
                    message: api_response.message,
                    data,
                    pagination: Some(api_response.pagination.into()),
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!("❌ Failed to retrieve cards: {e:?} 🚨");

                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip_all, fields(card_id = request.get_ref().card_id))]
    async fn find_by_id_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        let req = request.into_inner();

        info!(
            "📥 Received find_by_id_card request for card_id: {}",
            req.card_id
        );

        match self.query.find_by_id(req.card_id).await {
            Ok(api_response) => {
                info!("✅ Successfully found card with ID: {} 🃏", req.card_id);

                let reply = ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(reply))
            }
            Err(e) => {
                error!(
                    "❌ Failed to find card by ID: {e:?} 🚨",
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip_all, fields(user_id = request.get_ref().user_id))]
    async fn find_by_user_id_card(
        &self,
        request: Request<FindByUserIdCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        let req = request.into_inner();

        info!(
            "📥 Received find_by_user_id_card request for user_id: {}",
            req.user_id
        );

        match self.query.find_by_user_id(req.user_id).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully retrieved card for user_id: {} 👤",
                    req.user_id
                );

                let reply = ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!(
                    error = %e,
                    user_id = req.user_id,
                    "❌ Failed to find card by user ID: {e:?} 🚨",

                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip_all, fields(
        page = request.get_ref().page, 
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_active_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCardDeleteAt>, Status> {
        let req = request.into_inner();

         info!("📥 Received find_by_active_card request");

        let domain_req = FindAllCards {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                let data: Vec<genproto::card::CardResponseDeleteAt> = api_response.data.into_iter().map(Into::into).collect();
                let total_items = api_response.pagination.total_items;

                info!(
                    "✅ Successfully retrieved {} cards 📊 Total items: {total_items}",
                    data.clone().len(),
                );

                let reply = ApiResponsePaginationCardDeleteAt {
                    data,
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!("❌ Failed to retrieve cards: {e:?} 🚨");

                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip_all, fields(
        page = request.get_ref().page, 
        page_size = request.get_ref().page_size,
        search = tracing::field::Empty
    ))]
    async fn find_by_trashed_card(
        &self,
        request: Request<FindAllCardRequest>,
    ) -> Result<Response<ApiResponsePaginationCardDeleteAt>, Status> {
        let req = request.into_inner();

        info!("📥 Received find_by_trashed_card request");

        let domain_req = FindAllCards {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let data: Vec<genproto::card::CardResponseDeleteAt> = api_response.data.into_iter().map(Into::into).collect();
                let total_items = api_response.pagination.total_items;

                info!(
                    "✅ Successfully retrieved {} cards 📊 Total items: {total_items}",
                    data.clone().len(),
                );

                let reply = ApiResponsePaginationCardDeleteAt {
                    data,
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!("❌ Failed to retrieve cards: {e:?} 🚨");

                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
    #[instrument(skip_all, fields(card_number = request.get_ref().card_number))]
    async fn find_by_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        let req = request.into_inner();

        info!("📥 Received find_by_card_number request for card: {} 🔢", req.card_number);

        match self.query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                let reply = ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!("❌ Failed to find card by number: {e:?} 🚨");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all)]
    async fn dashboard_card(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseDashboardCard>, Status> {
        match self.dashboard.get_dashboard().await {
            Ok(api_response) => {
                info!("✅ Successfully fetched dashboard card, status={}", api_response.status);

                let reply = ApiResponseDashboardCard {
                    message: api_response.message,
                    status: api_response.status,
                    data: Some(api_response.data.into()),
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!("❌ Failed to fetch dashboard card, error={e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_number = request.get_ref().card_number))]
    async fn dashboard_card_number(
        &self,
        request: Request<FindByCardNumberRequest>,
    ) -> Result<Response<ApiResponseDashboardCardNumber>, Status> {
        let req = request.into_inner();

        match self.dashboard.get_dashboard_bycard(req.card_number.clone()).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched dashboard card by number={}, status={}",
                    req.card_number, api_response.status
                );

                let grpc_response = ApiResponseDashboardCardNumber {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch dashboard card by number={}, error={e:?}",
                    req.card_number, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }


    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_balance(
        &self,
        request: Request<FindYearBalance>,
    ) -> Result<Response<ApiResponseMonthlyBalance>, Status> {
        let req = request.into_inner();

        match self.stats.balance.get_monthly_balance(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly balance for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly balance for year={}, error={e:?}", req.year);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_balance(
        &self,
        request: Request<FindYearBalance>,
    ) -> Result<Response<ApiResponseYearlyBalance>, Status> {
        let req = request.into_inner();

        match self.stats.balance.get_yearly_balance(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly balance for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly balance for year={}, error={e:?}", req.year);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_topup_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.topup.get_monthly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly topup amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly topup amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_topup_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.topup.get_yearly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly topup amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly topup amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_withdraw_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.withdraw.get_monthly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly withdraw amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly withdraw amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_withdraw_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.withdraw.get_yearly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly withdraw amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly withdraw amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_transaction_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transaction.get_monthly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly transaction amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly transaction amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_transaction_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transaction.get_yearly_amount(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly transaction amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly transaction amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_transfer_sender_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transfer.get_monthly_amount_sender(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly transfer sender amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly transfer sender amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_transfer_sender_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transfer.get_yearly_amount_sender(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly transfer sender amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly transfer sender amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_monthly_transfer_receiver_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transfer.get_monthly_amount_receiver(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly transfer receiver amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly transfer receiver amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year))]
    async fn find_yearly_transfer_receiver_amount(
        &self,
        request: Request<FindYearAmount>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();

        match self.stats.transfer.get_yearly_amount_receiver(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly transfer receiver amount for year={}, status={}",
                    req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly transfer receiver amount for year={}, error={e:?}",
                    req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year, card_number = request.get_ref().card_number))]
    async fn find_monthly_balance_by_card_number(
        &self,
        request: Request<FindYearBalanceCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyBalance>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.balance.get_monthly_balance(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly balance by card_number={} for year={}, status={}",
                    domain_req.card_number, domain_req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly balance by card_number={} for year={}, error={e:?}",
                    domain_req.card_number, domain_req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year, card_number = request.get_ref().card_number))]
    async fn find_yearly_balance_by_card_number(
        &self,
        request: Request<FindYearBalanceCardNumber>,
    ) -> Result<Response<ApiResponseYearlyBalance>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.balance.get_yearly_balance(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly balance by card_number={} for year={}, status={}",
                    domain_req.card_number, domain_req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyBalance {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly balance by card_number={} for year={}, error={e:?}",
                    domain_req.card_number, domain_req.year,
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year, card_number = request.get_ref().card_number))]
    async fn find_monthly_topup_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.topup.get_monthly_amount(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched monthly topup amount by card_number={} for year={}, status={}",
                    domain_req.card_number, domain_req.year, api_response.status
                );

                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly topup amount by card_number={} for year={}, error={e:?}",
                    domain_req.card_number, domain_req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(year = request.get_ref().year, card_number = request.get_ref().card_number))]
    async fn find_yearly_topup_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.topup.get_yearly_amount(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched yearly topup amount by card_number={} for year={}, status={}",
                    domain_req.card_number, domain_req.year, api_response.status
                );

                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch yearly topup amount by card_number={} for year={}, error={e:?}",
                    domain_req.card_number, domain_req.year, 
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_monthly_withdraw_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.withdraw.get_monthly_amount(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_monthly_withdraw_amount_by_card_number");
                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_monthly_withdraw_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_yearly_withdraw_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.withdraw.get_yearly_amount(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_yearly_withdraw_amount_by_card_number");
                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_yearly_withdraw_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_monthly_transaction_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transaction.get_monthly_amount(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_monthly_transaction_amount_by_card_number");
                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_monthly_transaction_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_yearly_transaction_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transaction.get_yearly_amount(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_yearly_transaction_amount_by_card_number");
                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_yearly_transaction_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_monthly_transfer_sender_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transfer.get_monthly_amount_sender(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_monthly_transfer_sender_amount_by_card_number");
                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_monthly_transfer_sender_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_yearly_transfer_sender_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transfer.get_yearly_amount_sender(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_yearly_transfer_sender_amount_by_card_number");
                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_yearly_transfer_sender_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_monthly_transfer_receiver_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transfer.get_monthly_amount_receiver(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_monthly_transfer_receiver_amount_by_card_number");
                let grpc_response = ApiResponseMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_monthly_transfer_receiver_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(card_number = %request.get_ref().card_number, year = request.get_ref().year)
    )]
    async fn find_yearly_transfer_receiver_amount_by_card_number(
        &self,
        request: Request<FindYearAmountCardNumber>,
    ) -> Result<Response<ApiResponseYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearCardNumberCard {
            card_number: req.card_number.clone(),
            year: req.year,
        };

        match self.statsbycard.transfer.get_yearly_amount_receiver(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Success find_yearly_transfer_receiver_amount_by_card_number");
                let grpc_response = ApiResponseYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Error find_yearly_transfer_receiver_amount_by_card_number: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(
        skip_all,
        fields(user_id = request.get_ref().user_id, card_type = ?request.get_ref().card_type)
    )]
    async fn create_card(
        &self,
        request: Request<CreateCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_date(req.expire_date)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainCreateCardRequest {
            user_id: req.user_id,
            card_type: req.card_type,
            expire_date: date,
            cvv: req.cvv,
            card_provider: req.card_provider,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Successfully created card for user_id={}", domain_req.user_id);
                let grpc_response = ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to create card for user_id={}: {e:?}", domain_req.user_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_id = request.get_ref().card_id, user_id = request.get_ref().user_id))]
    async fn update_card(
        &self,
        request: Request<UpdateCardRequest>,
    ) -> Result<Response<ApiResponseCard>, Status> {
        let req = request.into_inner();

        let date = timestamp_to_naive_date(req.expire_date)
            .ok_or_else(|| Status::invalid_argument("expire_date invalid"))?;

        let domain_req = DomainUpdateCardRequest {
            card_id: req.card_id,
            user_id: req.user_id,
            card_type: req.card_type,
            expire_date: date,
            cvv: req.cvv,
            card_provider: req.card_provider,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Successfully updated card_id={}", domain_req.card_id);
                let grpc_response = ApiResponseCard {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to update card_id={}: {e:?}", domain_req.card_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_id = request.get_ref().card_id))]
    async fn trashed_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.trash(req.card_id).await {
            Ok(api_response) => {
                info!("✅ Trashed card_id={}", req.card_id);
                let grpc_response = ApiResponseCardDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to trash card_id={}: {e:?}", req.card_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_id = request.get_ref().card_id))]
    async fn restore_card(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDeleteAt>, Status> {
        let req = request.into_inner();

        match self.command.restore(req.card_id).await {
            Ok(api_response) => {
                info!("✅ Restored card_id={}", req.card_id);
                let grpc_response = ApiResponseCardDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore card_id={}: {e:?}", req.card_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all, fields(card_id = request.get_ref().card_id))]
    async fn delete_card_permanent(
        &self,
        request: Request<FindByIdCardRequest>,
    ) -> Result<Response<ApiResponseCardDelete>, Status> {
        let req = request.into_inner();

        match self.command.delete(req.card_id).await {
            Ok(api_response) => {
                info!("✅ Permanently deleted card_id={}", req.card_id);
                let grpc_response = ApiResponseCardDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete card_id={}: {e:?}", req.card_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all)]
    async fn restore_all_card(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseCardAll>, Status> {
        match self.command.restore_all().await {
            Ok(api_response) => {
                info!("✅ Restored all trashed cards");
                let grpc_response = ApiResponseCardAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to restore all trashed cards: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip_all)]
    async fn delete_all_card_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseCardAll>, Status> {
        match self.command.delete_all().await {
            Ok(api_response) => {
                info!("✅ Permanently deleted all cards");
                let grpc_response = ApiResponseCardAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all cards: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
