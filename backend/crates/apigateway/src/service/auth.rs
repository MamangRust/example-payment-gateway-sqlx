use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use genproto::auth::{
    ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister, GetMeRequest,
    LoginRequest, RefreshTokenRequest, RegisterRequest as ProtoRegisterRequest,
    auth_service_client::AuthServiceClient,
};
use opentelemetry::KeyValue;
use shared::{
    abstract_trait::auth::http::AuthGrpcClientTrait,
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::auth::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::{AppErrorGrpc, HttpError},
    observability::{Method, TracingMetrics},
};
use tonic::{Request, transport::Channel};
use tracing::{error, info};

pub struct AuthGrpcClientService {
    client: AuthServiceClient<Channel>,
    tracing_metrics_core: TracingMetrics,
    cache_store: Arc<CacheStore>,
}

impl AuthGrpcClientService {
    pub fn new(client: AuthServiceClient<Channel>, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            client,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl AuthGrpcClientTrait for AuthGrpcClientService {
    async fn login(&self, req: &AuthRequest) -> Result<ApiResponse<TokenResponse>, HttpError> {
        info!("Attempting login for email={}", req.email);

        let method = Method::Post;

        let email = req.email.clone();
        let password = req.password.clone();

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "LoginUser",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "login"),
                KeyValue::new("email", email.clone()),
            ],
        );

        let mut request = Request::new(LoginRequest { email, password });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().login_user(request).await {
            Ok(resp) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully logged in user")
                    .await;
                info!("✅ gRPC login_user request succeeded for email",);
                resp
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to log in user")
                    .await;
                error!("❌ gRPC login_user request failed: {}", status);
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseLogin = response.into_inner();
        let proto_token = inner.data.ok_or_else(|| {
            error!("❌ gRPC login_user returned empty token response");
            AppErrorGrpc::Unhandled("Missing token".into())
        })?;

        let domain_token: TokenResponse = proto_token.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_token,
        })
    }

    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, HttpError> {
        let method = Method::Get;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMe",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "get_me"),
                KeyValue::new("user_id", id.to_string()),
            ],
        );

        let cache_key = format!("auth:get_me:{id}");

        if let Some(cache) = self
            .cache_store
            .get_from_cache::<ApiResponse<UserResponse>>(&cache_key)
            .await
        {
            let log_msg = format!("✅ get_me cache hit for user_id={id}");
            info!("{log_msg}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, &log_msg)
                .await;
            return Ok(cache);
        }

        let mut request = Request::new(GetMeRequest { id });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().get_me(request).await {
            Ok(resp) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Successfully fetched user details",
                    )
                    .await;
                info!("✅ gRPC get_me succeeded for id={}", id);
                resp
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to fetch user details")
                    .await;
                error!("❌ gRPC get_me failed for id={}: {}", id, status);
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseGetMe = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!("❌ gRPC get_me returned empty user data for id={}", id);
            AppErrorGrpc::Unhandled("Missing user data".into())
        })?;

        let api_response = ApiResponse {
            status: inner.status,
            message: inner.message,
            data: proto_user.into(),
        };

        self.cache_store
            .set_to_cache(&cache_key, &api_response, Duration::minutes(10))
            .await;

        Ok(api_response)
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<ApiResponse<TokenResponse>, HttpError> {
        let method = Method::Post;

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "RefreshToken",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "refresh_token"),
            ],
        );

        let mut request = Request::new(RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().refresh_token(request).await {
            Ok(resp) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully refreshed token")
                    .await;
                info!("🔄 gRPC refresh_token request succeeded");
                resp
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to refresh token")
                    .await;
                error!("❌ gRPC refresh_token request failed: {}", status);
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseRefreshToken = response.into_inner();
        let proto_token = inner.data.ok_or_else(|| {
            error!("❌ gRPC refresh_token returned empty token data");

            HttpError::Internal("Missing token".into())
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
    ) -> Result<ApiResponse<UserResponse>, HttpError> {
        let method = Method::Post;

        let email = request.email.clone();

        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "RegisterUser",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "register"),
                KeyValue::new("email", email.clone()),
            ],
        );

        let mut req = Request::new(ProtoRegisterRequest {
            firstname: request.firstname.clone(),
            lastname: request.lastname.clone(),
            email,
            password: request.password.clone(),
            confirm_password: request.confirm_password.clone(),
        });

        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut req);

        let response = match self.client.clone().register_user(req).await {
            Ok(resp) => {
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Successfully registered user")
                    .await;
                info!("🎉 gRPC register_user succeeded for email");
                resp
            }
            Err(status) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Failed to register user")
                    .await;
                error!("❌ gRPC register_user failed for email: {}", status);
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseRegister = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!("❌ gRPC register_user returned empty user data for email");
            AppErrorGrpc::Unhandled("Missing user data".into())
        })?;

        let domain_user: UserResponse = proto_user.into();

        Ok(ApiResponse {
            status: inner.status,
            message: inner.message,
            data: domain_user,
        })
    }
}
