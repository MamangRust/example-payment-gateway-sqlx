use async_trait::async_trait;
use genproto::auth::{
    ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister, GetMeRequest,
    LoginRequest, RefreshTokenRequest, RegisterRequest as ProtoRegisterRequest,
    auth_service_client::AuthServiceClient,
};
use shared::{
    abstract_trait::auth::http::AuthGrpcClientTrait,
    domain::{
        requests::auth::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::{AppErrorGrpc, AppErrorHttp},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, transport::Channel};
use tracing::{error, info};

#[derive(Debug)]
pub struct AuthGrpcClientService {
    client: Arc<Mutex<AuthServiceClient<Channel>>>,
}

impl AuthGrpcClientService {
    pub async fn new(client: Arc<Mutex<AuthServiceClient<Channel>>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AuthGrpcClientTrait for AuthGrpcClientService {
    async fn login(&self, req: &AuthRequest) -> Result<ApiResponse<TokenResponse>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let email = req.email.clone();

        let request = Request::new(LoginRequest {
            email: req.email.clone(),
            password: req.password.clone(),
        });

        let response = match client.login_user(request).await {
            Ok(resp) => {
                info!("✅ gRPC login_user request succeeded for email={email}",);
                resp
            }
            Err(status) => {
                error!("❌ gRPC login_user request failed: {}", status);
                return Err(AppErrorHttp(AppErrorGrpc::from(status)));
            }
        };

        let inner: ApiResponseLogin = response.into_inner();
        let proto_token = inner.data.ok_or_else(|| {
            error!("❌ gRPC login_user returned empty token response");
            AppErrorHttp(AppErrorGrpc::Unhandled("Missing token".into()))
        })?;

        let domain_token: TokenResponse = proto_token.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_token,
        })
    }

    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, AppErrorHttp> {
        let mut client = self.client.lock().await;
        let request = Request::new(GetMeRequest { id });

        let response = match client.get_me(request).await {
            Ok(resp) => {
                info!("✅ gRPC get_me request succeeded for id={}", id);
                resp
            }
            Err(status) => {
                error!("❌ gRPC get_me request failed for id={}: {}", id, status);
                return Err(AppErrorHttp(AppErrorGrpc::from(status)));
            }
        };

        let inner: ApiResponseGetMe = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!("❌ gRPC get_me returned empty user data for id={}", id);
            AppErrorHttp(AppErrorGrpc::Unhandled("Missing user data".into()))
        })?;

        let domain_user: UserResponse = proto_user.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_user,
        })
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<ApiResponse<TokenResponse>, AppErrorHttp> {
        let mut client = self.client.lock().await;
        let request = Request::new(RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        });

        let response = match client.refresh_token(request).await {
            Ok(resp) => {
                info!("🔄 gRPC refresh_token request succeeded");
                resp
            }
            Err(status) => {
                error!("❌ gRPC refresh_token request failed: {}", status);
                return Err(AppErrorHttp(AppErrorGrpc::from(status)));
            }
        };

        let inner: ApiResponseRefreshToken = response.into_inner();
        let proto_token = inner.data.ok_or_else(|| {
            error!("❌ gRPC refresh_token returned empty token data");
            AppErrorHttp(AppErrorGrpc::Unhandled("Missing token".into()))
        })?;

        let domain_token: TokenResponse = proto_token.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_token,
        })
    }

    async fn register(
        &self,
        request: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, AppErrorHttp> {
        let mut client = self.client.lock().await;

        let req = Request::new(ProtoRegisterRequest {
            firstname: request.firstname.clone(),
            lastname: request.lastname.clone(),
            email: request.email.clone(),
            password: request.password.clone(),
            confirm_password: request.confirm_password.clone(),
        });

        let response = match client.register_user(req).await {
            Ok(resp) => {
                info!(
                    "🎉 gRPC register_user succeeded for email={}",
                    request.email
                );
                resp
            }
            Err(status) => {
                error!(
                    "❌ gRPC register_user failed for email={}: {}",
                    request.email, status
                );
                return Err(AppErrorHttp(AppErrorGrpc::from(status)));
            }
        };

        let inner: ApiResponseRegister = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!(
                "❌ gRPC register_user returned empty user data for email={}",
                request.email
            );
            AppErrorHttp(AppErrorGrpc::Unhandled("Missing user data".into()))
        })?;

        let domain_user: UserResponse = proto_user.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_user,
        })
    }
}
