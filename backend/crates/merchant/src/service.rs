use genproto::merchant::{
    ApiResponseMerchant, ApiResponseMerchantAll, ApiResponseMerchantDelete,
    ApiResponseMerchantDeleteAt, ApiResponseMerchantMonthlyAmount,
    ApiResponseMerchantMonthlyPaymentMethod, ApiResponseMerchantMonthlyTotalAmount,
    ApiResponseMerchantYearlyAmount, ApiResponseMerchantYearlyPaymentMethod,
    ApiResponseMerchantYearlyTotalAmount, ApiResponsePaginationMerchant,
    ApiResponsePaginationMerchantDeleteAt, ApiResponsePaginationMerchantTransaction,
    ApiResponsesMerchant, CreateMerchantRequest, FindAllMerchantApikey, FindAllMerchantRequest,
    FindAllMerchantTransaction, FindByApiKeyRequest, FindByIdMerchantRequest,
    FindByMerchantUserIdRequest, FindYearMerchant, FindYearMerchantByApikey, FindYearMerchantById,
    UpdateMerchantRequest, merchant_service_server::MerchantService,
};
use std::sync::Arc;

use crate::di::DependenciesInject;
use shared::{
    domain::requests::merchant::{
        CreateMerchantRequest as DomainCreateMerchantRequest, FindAllMerchantTransactions,
        FindAllMerchantTransactionsByApiKey, FindAllMerchantTransactionsById, FindAllMerchants,
        MonthYearAmountApiKey, MonthYearAmountMerchant, MonthYearPaymentMethodApiKey,
        MonthYearPaymentMethodMerchant, MonthYearTotalAmountApiKey, MonthYearTotalAmountMerchant,
        UpdateMerchantRequest as DomainUpdateMerchantRequest,
    },
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct MerchantServiceImpl {
    pub di: Arc<DependenciesInject>,
}

impl MerchantServiceImpl {
    pub fn new(di: Arc<DependenciesInject>) -> Self {
        Self { di }
    }
}

#[tonic::async_trait]
impl MerchantService for MerchantServiceImpl {
    async fn find_all_merchant(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchant>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.merchant_query.find_all(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchant {
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

    async fn find_by_id_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        let req = request.into_inner();

        match self.di.merchant_query.find_by_id(req.merchant_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_api_key(
        &self,
        request: Request<FindByApiKeyRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        let req = request.into_inner();

        match self.di.merchant_query.find_by_apikey(&req.api_key).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_all_transaction_merchant(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchantTransactions {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self
            .di
            .merchant_transaction
            .find_all(&FindAllMerchantTransactions {
                page: domain_req.page,
                page_size: domain_req.page_size,
                search: domain_req.search,
            })
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchantTransaction {
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

    async fn find_monthly_payment_methods_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_method
            .get_monthly_method(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_payment_method_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_method
            .get_yearly_method(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_amount
            .get_monthly_amount(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_amount
            .get_yearly_amount(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_total_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_total_amount
            .get_monthly_total_amount(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_total_amount_merchant(
        &self,
        request: Request<FindYearMerchant>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_stats_total_amount
            .get_yearly_total_amount(req.year)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_all_transaction_by_merchant(
        &self,
        request: Request<FindAllMerchantTransaction>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchantTransactionsById {
            merchant_id: req.merchant_id,
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self
            .di
            .merchant_transaction
            .find_all_by_id(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchantTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: None,
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_payment_method_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethodMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_method_by_merchant
            .find_monthly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_payment_method_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethodMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_method_by_merchant
            .find_yearly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearAmountMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_amount_by_merchant
            .find_monthly_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearAmountMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_amount_by_merchant
            .find_yearly_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_total_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearTotalAmountMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_total_amount_by_merchant
            .find_monthly_total_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_total_amount_by_merchants(
        &self,
        request: Request<FindYearMerchantById>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearTotalAmountMerchant {
            merchant_id: req.merchant_id,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_total_amount_by_merchant
            .find_yearly_total_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_all_transaction_by_apikey(
        &self,
        request: Request<FindAllMerchantApikey>,
    ) -> Result<Response<ApiResponsePaginationMerchantTransaction>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchantTransactionsByApiKey {
            api_key: req.api_key,
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self
            .di
            .merchant_transaction
            .find_all_by_api_key(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchantTransaction {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    pagination: None,
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_payment_method_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyPaymentMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethodApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_method_by_apikey
            .find_monthly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_payment_method_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyPaymentMethod>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearPaymentMethodApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_method_by_apikey
            .find_yearly_method(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyPaymentMethod {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearAmountApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_amount_by_apikey
            .find_monthly_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearAmountApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_amount_by_apikey
            .find_yearly_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_monthly_total_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantMonthlyTotalAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearTotalAmountApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_total_amount_by_apikey
            .find_monthly_total_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantMonthlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_yearly_total_amount_by_apikey(
        &self,
        request: Request<FindYearMerchantByApikey>,
    ) -> Result<Response<ApiResponseMerchantYearlyTotalAmount>, Status> {
        let req = request.into_inner();
        let domain_req = MonthYearTotalAmountApiKey {
            api_key: req.api_key,
            year: req.year,
        };

        match self
            .di
            .merchant_stats_total_amount_by_apikey
            .find_yearly_total_amount(&domain_req)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantYearlyTotalAmount {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_merchant_user_id(
        &self,
        request: Request<FindByMerchantUserIdRequest>,
    ) -> Result<Response<ApiResponsesMerchant>, Status> {
        let req = request.into_inner();

        match self
            .di
            .merchant_query
            .find_merchant_user_id(req.user_id)
            .await
        {
            Ok(api_response) => {
                let grpc_response = ApiResponsesMerchant {
                    data: api_response.data.into_iter().map(Into::into).collect(),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn find_by_active(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.merchant_query.find_active(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchantDeleteAt {
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

    async fn find_by_trashed(
        &self,
        request: Request<FindAllMerchantRequest>,
    ) -> Result<Response<ApiResponsePaginationMerchantDeleteAt>, Status> {
        let req = request.into_inner();
        let domain_req = FindAllMerchants {
            page: req.page,
            page_size: req.page_size,
            search: req.search,
        };

        match self.di.merchant_query.find_trashed(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponsePaginationMerchantDeleteAt {
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

    async fn create_merchant(
        &self,
        request: Request<CreateMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        let req = request.into_inner();
        let domain_req = DomainCreateMerchantRequest {
            user_id: req.user_id,
            name: req.name,
        };

        match self.di.merchant_command.create(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn update_merchant(
        &self,
        request: Request<UpdateMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchant>, Status> {
        let req = request.into_inner();
        let domain_req = DomainUpdateMerchantRequest {
            merchant_id: Some(req.merchant_id),
            user_id: req.user_id,
            name: req.name,
            status: "pending".to_string(),
        };

        match self.di.merchant_command.update(&domain_req).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchant {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn trashed_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDeleteAt>, Status> {
        let req = request.into_inner();

        match self.di.merchant_command.trash(req.merchant_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_merchant(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDeleteAt>, Status> {
        let req = request.into_inner();

        match self.di.merchant_command.restore(req.merchant_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantDeleteAt {
                    data: Some(api_response.data.into()),
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_merchant_permanent(
        &self,
        request: Request<FindByIdMerchantRequest>,
    ) -> Result<Response<ApiResponseMerchantDelete>, Status> {
        let req = request.into_inner();

        match self.di.merchant_command.delete(req.merchant_id).await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantDelete {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn restore_all_merchant(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseMerchantAll>, Status> {
        match self.di.merchant_command.restore_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }

    async fn delete_all_merchant_permanent(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ApiResponseMerchantAll>, Status> {
        match self.di.merchant_command.delete_all().await {
            Ok(api_response) => {
                let grpc_response = ApiResponseMerchantAll {
                    message: api_response.message,
                    status: api_response.status,
                };
                Ok(Response::new(grpc_response))
            }
            Err(e) => Err(AppErrorGrpc::from(e).into()),
        }
    }
}
