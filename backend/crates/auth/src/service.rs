use genproto::auth::{
    ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister, GetMeRequest,
    LoginRequest, RefreshTokenRequest, RegisterRequest, auth_service_server::AuthService,
};
use shared::{
    abstract_trait::auth::service::DynAuthService,
    domain::requests::auth::{AuthRequest, RegisterRequest as RegisterDomainRequest},
    errors::AppErrorGrpc,
};
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub auth_service: DynAuthService,
}

impl AuthServiceImpl {
    pub fn new(auth: DynAuthService) -> Self {
        Self { auth_service: auth }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
    async fn register_user(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<ApiResponseRegister>, Status> {
        let req = request.into_inner();

        let domain_req = RegisterDomainRequest {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email,
            password: req.password,
            confirm_password: req.confirm_password,
        };

        let api_response = self
            .auth_service
            .register_user(&domain_req)
            .await
            .map_err(AppErrorGrpc::from)?;

        let reply = ApiResponseRegister {
            status: api_response.status,
            message: api_response.message,
            data: Some(api_response.data.into()),
        };

        info!("User registered successfully: {}", reply.message);
        Ok(Response::new(reply))
    }

    async fn login_user(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<ApiResponseLogin>, Status> {
        let req = request.into_inner();

        let domain_req = AuthRequest {
            email: req.email,
            password: req.password,
        };

        let api_response = self
            .auth_service
            .login_user(&domain_req)
            .await
            .map_err(AppErrorGrpc::from)?;

        let reply = ApiResponseLogin {
            status: api_response.status,
            message: api_response.message,
            data: Some(api_response.data.into()),
        };

        info!("User login success: {}", reply.message);
        Ok(Response::new(reply))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<ApiResponseRefreshToken>, Status> {
        let req = request.into_inner();

        let api_response = self
            .auth_service
            .refresh_token(&req.refresh_token)
            .await
            .map_err(AppErrorGrpc::from)?;

        let reply = ApiResponseRefreshToken {
            status: api_response.status,
            message: api_response.message,
            data: Some(api_response.data.into()),
        };

        info!("Refresh token success: {}", reply.message);
        Ok(Response::new(reply))
    }

    async fn get_me(
        &self,
        request: Request<GetMeRequest>,
    ) -> Result<Response<ApiResponseGetMe>, Status> {
        let req = request.into_inner();

        let api_response = self
            .auth_service
            .get_me(req.id)
            .await
            .map_err(AppErrorGrpc::from)?;

        let reply = ApiResponseGetMe {
            status: api_response.status,
            message: api_response.message,
            data: Some(api_response.data.into()),
        };

        info!("GetMe success: {}", reply.message);
        Ok(Response::new(reply))
    }
}
