use genproto::topup::{
    ApiResponsePaginationTopup, ApiResponsePaginationTopupDeleteAt, ApiResponseTopup,
    ApiResponseTopupAll, ApiResponseTopupDelete, ApiResponseTopupDeleteAt,
    ApiResponseTopupMonthAmount, ApiResponseTopupMonthMethod, ApiResponseTopupMonthStatusFailed,
    ApiResponseTopupMonthStatusSuccess, ApiResponseTopupYearAmount, ApiResponseTopupYearMethod,
    ApiResponseTopupYearStatusFailed, ApiResponseTopupYearStatusSuccess, ApiResponsesTopup,
    CreateTopupRequest, FindAllTopupByCardNumberRequest, FindAllTopupRequest,
    FindByCardNumberTopupRequest, FindByIdTopupRequest, FindMonthlyTopupStatus,
    FindMonthlyTopupStatusCardNumber, FindYearTopupCardNumber, FindYearTopupStatus,
    FindYearTopupStatusCardNumber, UpdateTopupRequest, topup_service_server::TopupService,
};
use shared::{
    abstract_trait::topup::service::{
        command::DynTopupCommandService,
        query::DynTopupQueryService,
        stats::{
            amount::DynTopupStatsAmountService, method::DynTopupStatsMethodService,
            status::DynTopupStatsStatusService,
        },
        statsbycard::{
            amount::DynTopupStatsAmountByCardService, method::DynTopupStatsMethodByCardService,
            status::DynTopupStatsStatusByCardService,
        },
    },
    domain::requests::topup::{
        CreateTopupRequest as DomainCreateTopupRequest, FindAllTopups, FindAllTopupsByCardNumber,
        MonthTopupStatus, MonthTopupStatusCardNumber,
        UpdateTopupRequest as DomainUpdateTopupRequst, YearMonthMethod, YearTopupStatusCardNumber,
    },
    errors::AppErrorGrpc,
    utils::{mask_api_key, mask_card_number},
};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

#[derive(Clone)]
pub struct TopupStats {
    pub amount: DynTopupStatsAmountService,
    pub method: DynTopupStatsMethodService,
    pub status: DynTopupStatsStatusService,
}

#[derive(Clone)]
pub struct TopupStatsByCard {
    pub amount: DynTopupStatsAmountByCardService,
    pub method: DynTopupStatsMethodByCardService,
    pub status: DynTopupStatsStatusByCardService,
}

#[derive(Clone)]
pub struct TopupServiceImpl {
    pub query: DynTopupQueryService,
    pub command: DynTopupCommandService,
    pub stats: TopupStats,
    pub statsbycard: TopupStatsByCard,
}

impl TopupServiceImpl {
    pub fn new(
        query: DynTopupQueryService,
        command: DynTopupCommandService,
        stats: TopupStats,
        statsbycard: TopupStatsByCard,
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
impl TopupService for TopupServiceImpl {
    #[instrument(skip(self, request), level = "info")]
    async fn find_all_topup(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_all_topup - page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopup {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_all_topup succeeded - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_all_topup failed: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_all_topup_by_card_number(
        &self,
        request: Request<FindAllTopupByCardNumberRequest>,
    ) -> Result<Response<ApiResponsePaginationTopup>, Status> {
        let req = request.into_inner();
        let masked_card = mask_api_key(&req.card_number);
        info!(
            "handling find_all_topup_by_card_number - card: {masked_card}, page: {}, page_size: {}, search: {:?}",
            req.page, req.page_size, req.search
        );

        let domain_req = FindAllTopupsByCardNumber {
            card_number: req.card_number,
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_all_by_card_number(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationTopup {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_all_topup_by_card_number succeeded for card {masked_card} - returned {} records",
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_all_topup_by_card_number failed for card {masked_card}: {e:?}");
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_id_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();
        info!("handling find_by_id_topup - topup_id: {}", req.topup_id);

        match self.query.find_by_id(req.topup_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!("find_by_id_topup succeeded for id: {}", req.topup_id);
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("find_by_id_topup failed for id {}: {e:?}", req.topup_id);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_status_success(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_monthly_topup_status_success - year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthTopupStatus {
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
                let grpc_response = ApiResponseTopupMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_topup_status_success succeeded for year {} month {} - returned {} records",
                    req.year,
                    req.month,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_topup_status_success failed for year {} month {}: {e:?}",
                    req.year, req.month
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_status_success(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_yearly_topup_status_success - year: {}",
            req.year
        );

        match self.stats.status.get_yearly_status_success(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_yearly_topup_status_success succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_yearly_topup_status_success failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_status_failed(
        &self,
        request: Request<FindMonthlyTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_monthly_topup_status_failed - year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthTopupStatus {
            year: req.year,
            month: req.month,
        };

        match self.stats.status.get_month_status_failed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_topup_status_failed succeeded for year {} month {} - returned {} records",
                    req.year,
                    req.month,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_topup_status_failed failed for year {} month {}: {e:?}",
                    req.year, req.month
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_status_failed(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
        let req = request.into_inner();
        info!(
            "handling find_yearly_topup_status_failed - year: {}",
            req.year
        );

        match self.stats.status.get_yearly_status_failed(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_yearly_topup_status_failed succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_yearly_topup_status_failed failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_status_success_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusSuccess>, Status> {
        let req = request.into_inner();

        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling find_monthly_topup_status_success_by_card_number - card: {masked_card}, year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthTopupStatusCardNumber {
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
                let grpc_response = ApiResponseTopupMonthStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_topup_status_success_by_card_number succeeded for card {masked_card} year {} month {} - returned {} records",
                    req.year,
                    req.month,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_topup_status_success_by_card_number failed for card {masked_card} year {} month {}: {e:?}",
                    req.year, req.month
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_status_success_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusSuccess>, Status> {
        let req = request.into_inner();
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling find_yearly_topup_status_success_by_card_number - card: {masked_card}, year: {}",
            req.year
        );

        let domain_req = YearTopupStatusCardNumber {
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
                let grpc_response = ApiResponseTopupYearStatusSuccess {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_yearly_topup_status_success_by_card_number succeeded for card {masked_card} year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_yearly_topup_status_success_by_card_number failed for card {masked_card} year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindMonthlyTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthStatusFailed>, Status> {
        let req = request.into_inner();
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling find_monthly_topup_status_failed_by_card_number - card: {masked_card}, year: {}, month: {}",
            req.year, req.month
        );

        let domain_req = MonthTopupStatusCardNumber {
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
                let grpc_response = ApiResponseTopupMonthStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_topup_status_failed_by_card_number succeeded for card {masked_card} year {} month {} - returned {} records",
                    req.year,
                    req.month,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_topup_status_failed_by_card_number failed for card {masked_card} year {} month {}: {e:?}",
                    req.year, req.month
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_status_failed_by_card_number(
        &self,
        request: Request<FindYearTopupStatusCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearStatusFailed>, Status> {
        let req = request.into_inner();
        let masked_card = mask_card_number(&req.card_number);
        info!(
            "handling find_yearly_topup_status_failed_by_card_number - card: {masked_card}, year: {}",
            req.year
        );

        let domain_req = YearTopupStatusCardNumber {
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
                let grpc_response = ApiResponseTopupYearStatusFailed {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_yearly_topup_status_failed_by_card_number succeeded for card {masked_card} year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_yearly_topup_status_failed_by_card_number failed for card {masked_card} year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
        let req = request.into_inner();
        info!("handling find_monthly_topup_methods - year: {}", req.year);

        match self.stats.method.get_monthly_methods(req.year).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseTopupMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                info!(
                    "find_monthly_topup_methods succeeded for year {} - returned {} records",
                    req.year,
                    grpc_response.data.len()
                );
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "find_monthly_topup_methods failed for year {}: {e:?}",
                    req.year
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_methods(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly top-up methods for year: {}", req.year);

        match self.stats.method.get_yearly_methods(req.year).await {
            Ok(api_response) => {
                info!(
                    "✅ Successfully fetched {} yearly top-up methods",
                    api_response.data.len()
                );
                let grpc_response = ApiResponseTopupYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly top-up methods: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]

    async fn find_monthly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching monthly top-up amounts for year: {}", req.year);

        match self.stats.amount.get_monthly_amounts(req.year).await {
            Ok(api_response) => {
                info!("✅ Successfully fetched monthly top-up amounts");
                let grpc_response = ApiResponseTopupMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly top-up amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_amounts(
        &self,
        request: Request<FindYearTopupStatus>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
        let req = request.into_inner();
        info!("📊 Fetching yearly top-up amounts for year: {}", req.year);

        match self.stats.amount.get_yearly_amounts(req.year).await {
            Ok(api_response) => {
                info!("✅ Successfully fetched yearly top-up amounts");
                let grpc_response = ApiResponseTopupYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly top-up amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthMethod>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly top-up methods for card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .method
            .get_monthly_methods(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Successfully fetched monthly top-up methods for card");
                let grpc_response = ApiResponseTopupMonthMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch monthly top-up methods for card {}: {:?}",
                    domain_req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_methods_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearMethod>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly top-up methods | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearMonthMethod {
            card_number: req.card_number,
            year: req.year,
        };

        match self
            .statsbycard
            .method
            .get_yearly_methods(&domain_req)
            .await
        {
            Ok(api_response) => {
                info!("✅ Found {} yearly top-up methods", api_response.data.len());
                Ok(Response::new(ApiResponseTopupYearMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly top-up methods: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_monthly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupMonthAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching monthly top-up amounts | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearMonthMethod {
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
                info!("✅ Successfully fetched monthly top-up amounts");
                Ok(Response::new(ApiResponseTopupMonthAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch monthly top-up amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_yearly_topup_amounts_by_card_number(
        &self,
        request: Request<FindYearTopupCardNumber>,
    ) -> Result<Response<ApiResponseTopupYearAmount>, Status> {
        let req = request.into_inner();
        info!(
            "📊 Fetching yearly top-up amounts | card: {}, year: {}",
            req.card_number, req.year
        );

        let domain_req = YearMonthMethod {
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
                info!("✅ Successfully fetched yearly top-up amounts");
                Ok(Response::new(ApiResponseTopupYearAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch yearly top-up amounts: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_card_number_topup(
        &self,
        request: Request<FindByCardNumberTopupRequest>,
    ) -> Result<Response<ApiResponsesTopup>, Status> {
        let req = request.into_inner();
        info!("📊 Finding top-ups for card: {}", req.card_number);

        match self.query.find_by_card(&req.card_number).await {
            Ok(api_response) => {
                info!("✅ Found {} top-up records", api_response.data.len());
                Ok(Response::new(ApiResponsesTopup {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to fetch top-ups for card {}: {:?}",
                    req.card_number, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_active(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "📄 Fetching active topups | page: {}, page_size: {}",
            req.page, req.page_size
        );

        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_active(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} active topups", api_response.data.len());
                Ok(Response::new(ApiResponsePaginationTopupDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch active topups: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn find_by_trashed(
        &self,
        request: Request<FindAllTopupRequest>,
    ) -> Result<Response<ApiResponsePaginationTopupDeleteAt>, Status> {
        let req = request.into_inner();
        info!(
            "🗑️ Fetching trashed topups | page: {}, page_size: {}",
            req.page, req.page_size
        );

        let domain_req = FindAllTopups {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                info!("✅ Found {} trashed topups", api_response.data.len());
                Ok(Response::new(ApiResponsePaginationTopupDeleteAt {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: Some(api_response.pagination.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to fetch trashed topups: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn create_topup(
        &self,
        request: Request<CreateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();
        info!(
            "➕ Creating topup | card: {}, amount: {}",
            req.card_number, req.topup_amount
        );

        let domain_req = DomainCreateTopupRequest {
            card_number: req.card_number,
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        match self.command.create(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Topup created successfully (id: {})",
                    api_response.data.id
                );
                Ok(Response::new(ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to create topup: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn update_topup(
        &self,
        request: Request<UpdateTopupRequest>,
    ) -> Result<Response<ApiResponseTopup>, Status> {
        let req = request.into_inner();
        info!(
            "✏️ Updating topup | id: {}, new_amount: {}",
            req.topup_id, req.topup_amount
        );

        let domain_req = DomainUpdateTopupRequst {
            card_number: req.card_number,
            topup_id: Some(req.topup_id),
            topup_amount: req.topup_amount as i64,
            topup_method: req.topup_method,
        };

        match self.command.update(&domain_req).await {
            Ok(api_response) => {
                info!(
                    "✅ Topup updated successfully (id: {})",
                    api_response.data.id
                );
                Ok(Response::new(ApiResponseTopup {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to update topup {}: {:?}", req.topup_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn trashed_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        let req = request.into_inner();
        info!("🗑️ Trashing topup | id: {}", req.topup_id);

        match self.command.trashed(req.topup_id).await {
            Ok(api_response) => {
                info!("✅ Topup trashed successfully (id: {})", req.topup_id);
                Ok(Response::new(ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to trash topup {}: {:?}", req.topup_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn restore_topup(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDeleteAt>, Status> {
        let req = request.into_inner();
        info!("♻️ Restoring topup | id: {}", req.topup_id);

        match self.command.restore(req.topup_id).await {
            Ok(api_response) => {
                info!("✅ Topup restored successfully (id: {})", req.topup_id);
                Ok(Response::new(ApiResponseTopupDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to restore topup {}: {:?}", req.topup_id, e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self, request), level = "info")]
    async fn delete_topup_permanent(
        &self,
        request: Request<FindByIdTopupRequest>,
    ) -> Result<Response<ApiResponseTopupDelete>, Status> {
        let req = request.into_inner();
        info!("🔥 Permanently deleting topup | id: {}", req.topup_id);

        match self.command.delete_permanent(req.topup_id).await {
            Ok(api_response) => {
                info!("✅ Topup permanently deleted (id: {})", req.topup_id);
                Ok(Response::new(ApiResponseTopupDelete {
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!(
                    "❌ Failed to permanently delete topup {}: {:?}",
                    req.topup_id, e
                );
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn restore_all_topup(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        info!("♻️ Restoring all trashed topups");

        match self.command.restore_all().await {
            Ok(api_response) => {
                info!("✅ All topups restored successfully");
                Ok(Response::new(ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to restore all topups: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }

    #[instrument(skip(self), level = "info")]
    async fn delete_all_topup_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseTopupAll>, Status> {
        info!("🔥 Permanently deleting all topups");

        match self.command.delete_all().await {
            Ok(api_response) => {
                info!("✅ All topups permanently deleted");
                Ok(Response::new(ApiResponseTopupAll {
                    message: api_response.message,
                    status: api_response.status,
                }))
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all topups: {:?}", e);
                Err(AppErrorGrpc::from(e).into())
            }
        }
    }
}
