use anyhow::Result;
use async_trait::async_trait;
use genproto::auth::{
    ApiResponseGetMe, ApiResponseLogin, ApiResponseRefreshToken, ApiResponseRegister, GetMeRequest,
    LoginRequest, RefreshTokenRequest, RegisterRequest as ProtoRegisterRequest,
    auth_service_client::AuthServiceClient,
};
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use shared::{
    abstract_trait::auth::http::AuthGrpcClientTrait,
    domain::{
        requests::auth::{AuthRequest, RegisterRequest},
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::{AppErrorGrpc, HttpError},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
};
use tokio::time::Instant;
use tonic::{Request, transport::Channel};
use tracing::{error, info};

#[derive(Debug)]
pub struct AuthGrpcClientService {
    client: AuthServiceClient<Channel>,
    metrics: Metrics,
}

impl AuthGrpcClientService {
    pub fn new(client: AuthServiceClient<Channel>) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self { client, metrics })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("auth-service-client")
    }

    fn inject_trace_context<T>(&self, cx: &Context, request: &mut Request<T>) {
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(cx, &mut MetadataInjector(request.metadata_mut()))
        });
    }

    fn start_tracing(&self, operation_name: &str, attributes: Vec<KeyValue>) -> TracingContext {
        let start_time = Instant::now();
        let tracer = self.get_tracer();
        let mut span = tracer
            .span_builder(operation_name.to_string())
            .with_kind(SpanKind::Server)
            .with_attributes(attributes)
            .start(&tracer);

        info!("Starting operation: {operation_name}");

        span.add_event(
            "Operation started",
            vec![
                KeyValue::new("operation", operation_name.to_string()),
                KeyValue::new("timestamp", start_time.elapsed().as_secs_f64().to_string()),
            ],
        );

        let cx = Context::current_with_span(span);
        TracingContext { cx, start_time }
    }

    async fn complete_tracing_success(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, true, message)
            .await;
    }

    async fn complete_tracing_error(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        error_message: &str,
    ) {
        self.complete_tracing_internal(tracing_ctx, method, false, error_message)
            .await;
    }

    async fn complete_tracing_internal(
        &self,
        tracing_ctx: &TracingContext,
        method: Method,
        is_success: bool,
        message: &str,
    ) {
        let status_str = if is_success { "SUCCESS" } else { "ERROR" };
        let status = if is_success {
            StatusUtils::Success
        } else {
            StatusUtils::Error
        };
        let elapsed = tracing_ctx.start_time.elapsed().as_secs_f64();

        tracing_ctx.cx.span().add_event(
            "Operation completed",
            vec![
                KeyValue::new("status", status_str),
                KeyValue::new("duration_secs", elapsed.to_string()),
                KeyValue::new("message", message.to_string()),
            ],
        );

        if is_success {
            info!("Operation completed successfully: {message}");
        } else {
            error!("Operation failed: {message}");
        }

        self.metrics.record(method, status, elapsed);

        tracing_ctx.cx.span().end();
    }
}

#[async_trait]
impl AuthGrpcClientTrait for AuthGrpcClientService {
    async fn login(&self, req: &AuthRequest) -> Result<ApiResponse<TokenResponse>, HttpError> {
        info!("Attempting login for email={}", req.email);

        let method = Method::Post;

        let tracing_ctx = self.start_tracing(
            "LoginUser",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "login"),
                KeyValue::new("email", req.email.clone()),
            ],
        );

        let mut request = Request::new(LoginRequest {
            email: req.email.clone(),
            password: req.password.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().login_user(request).await {
            Ok(resp) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully logged in user")
                    .await;
                info!(
                    "✅ gRPC login_user request succeeded for email={}",
                    req.email
                );
                resp
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to log in user")
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

        let tracing_ctx = self.start_tracing(
            "GetMe",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "get_me"),
                KeyValue::new("user_id", id.to_string()),
            ],
        );

        let mut request = Request::new(GetMeRequest { id });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().get_me(request).await {
            Ok(resp) => {
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Successfully fetched user details",
                )
                .await;
                info!("✅ gRPC get_me request succeeded for id={}", id);
                resp
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to fetch user details")
                    .await;
                error!("❌ gRPC get_me request failed for id={}: {}", id, status);
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseGetMe = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!("❌ gRPC get_me returned empty user data for id={}", id);
            AppErrorGrpc::Unhandled("Missing user data".into())
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
    ) -> Result<ApiResponse<TokenResponse>, HttpError> {
        let method = Method::Post;

        let tracing_ctx = self.start_tracing(
            "RefreshToken",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "refresh_token"),
            ],
        );

        let mut request = Request::new(RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let response = match self.client.clone().refresh_token(request).await {
            Ok(resp) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully refreshed token")
                    .await;
                info!("🔄 gRPC refresh_token request succeeded");
                resp
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to refresh token")
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

        let tracing_ctx = self.start_tracing(
            "RegisterUser",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("operation", "register"),
                KeyValue::new("email", request.email.clone()),
            ],
        );

        let mut req = Request::new(ProtoRegisterRequest {
            firstname: request.firstname.clone(),
            lastname: request.lastname.clone(),
            email: request.email.clone(),
            password: request.password.clone(),
            confirm_password: request.confirm_password.clone(),
        });

        self.inject_trace_context(&tracing_ctx.cx, &mut req);

        let response = match self.client.clone().register_user(req).await {
            Ok(resp) => {
                self.complete_tracing_success(&tracing_ctx, method, "Successfully registered user")
                    .await;
                info!(
                    "🎉 gRPC register_user succeeded for email={}",
                    request.email
                );
                resp
            }
            Err(status) => {
                self.complete_tracing_error(&tracing_ctx, method, "Failed to register user")
                    .await;
                error!(
                    "❌ gRPC register_user failed for email={}: {}",
                    request.email, status
                );
                return Err(AppErrorGrpc::from(status).into());
            }
        };

        let inner: ApiResponseRegister = response.into_inner();
        let proto_user = inner.data.ok_or_else(|| {
            error!(
                "❌ gRPC register_user returned empty user data for email={}",
                request.email
            );
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
