use crate::{
    abstract_trait::{
        auth::service::AuthServiceTrait,
        hashing::DynHashing,
        jwt::DynJwtService,
        refresh_token::command::DynRefreshTokenCommandRepository,
        role::repository::query::DynRoleQueryRepository,
        token::DynTokenService,
        user::repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    domain::{
        requests::{
            auth::{AuthRequest, RegisterRequest},
            refresh_token::UpdateRefreshToken,
            user::CreateUserRequest,
            user_role::CreateUserRoleRequest,
        },
        responses::{ApiResponse, TokenResponse, UserResponse},
    },
    errors::ServiceError,
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AuthService {
    query: DynUserQueryRepository,
    command: DynUserCommandRepository,
    hashing: DynHashing,
    role: DynRoleQueryRepository,
    user_role: DynUserRoleCommandRepository,
    refresh_command: DynRefreshTokenCommandRepository,
    jwt_config: DynJwtService,
    token: DynTokenService,
    tracing_metrics_core: TracingMetrics,
    cache_store: Arc<CacheStore>,
}

impl std::fmt::Debug for AuthService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthService")
            .field("query", &"DynUserQueryRepository")
            .field("command", &"DynUserCommandRepository")
            .field("hashing", &"Hashing")
            .field("jwt_config", &"JwtConfig")
            .field("role", &"DynJwtService")
            .field("user_role", &"DynUserRoleService")
            .field("refresh_command", &"DynRefreshTokenCommandService")
            .field("token", &"DynTokenService")
            .finish()
    }
}

pub struct AuthServiceDeps {
    pub query: DynUserQueryRepository,
    pub command: DynUserCommandRepository,
    pub hashing: DynHashing,
    pub role: DynRoleQueryRepository,
    pub user_role: DynUserRoleCommandRepository,
    pub refresh_command: DynRefreshTokenCommandRepository,
    pub jwt_config: DynJwtService,
    pub token: DynTokenService,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl AuthService {
    pub fn new(deps: AuthServiceDeps) -> Result<Self> {
        let AuthServiceDeps {
            query,
            command,
            hashing,
            role,
            user_role,
            refresh_command,
            jwt_config,
            token,
            cache_store,
            tracing_metrics_core,
        } = deps;

        Ok(Self {
            query,
            command,
            hashing,
            role,
            user_role,
            refresh_command,
            jwt_config,
            token,
            cache_store,
            tracing_metrics_core,
        })
    }
}

#[async_trait]
impl AuthServiceTrait for AuthService {
    async fn register_user(
        &self,
        req: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!(
            "🆕 New user registration attempt with email: {}",
            &req.email.clone(),
        );

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "RegisterUser",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("user.email", req.email.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let cache_key = format!("auth:registered:{}", req.email);

        if let Some(cached_user) = self.cache_store.get_from_cache(&cache_key).await {
            let log_msg = format!(
                "✅ [REGISTER] Cache hit! User already registered | Email: {}",
                req.email
            );
            info!("{log_msg}");

            self.tracing_metrics_core
                .complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "User already registered (from cache)",
                )
                .await;

            return Ok(ApiResponse {
                status: "success".to_string(),
                message: "User already registered (from cache)".to_string(),
                data: cached_user,
            });
        }

        let existing_user = match self.query.find_by_email(req.email.clone()).await {
            Ok(user) => user,
            Err(e) => {
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        if existing_user.is_some() {
            let msg = "Email already exists";
            error!("❌ [REGISTER] Email already taken | Email: {}", req.email);
            self.tracing_metrics_core
                .complete_tracing_error(&tracing_ctx, method, msg)
                .await;
            return Err(ServiceError::Custom("Email already registered".to_string()));
        }

        let hashed_password = match self.hashing.hash_password(&req.password).await {
            Ok(hash) => hash,
            Err(e) => {
                error!("❌ Failed to hash password: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Failed to hash password")
                    .await;

                return Err(ServiceError::InternalServerError(
                    "Failed to hash password".into(),
                ));
            }
        };

        const DEFAULT_ROLE_NAME: &str = "ROLE_ADMIN";
        let role = match self.role.find_by_name(DEFAULT_ROLE_NAME).await {
            Ok(Some(role)) => role,
            Ok(None) => {
                error!("❌ Role not found: {}", DEFAULT_ROLE_NAME);
                return Err(ServiceError::Custom("Default role not found".to_string()));
            }
            Err(e) => {
                error!("❌ Failed to query role: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Role query failed")
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let new_request = CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            password: hashed_password,
            email: req.email.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        let new_user = match self.command.create(&new_request).await {
            Ok(user) => user,
            Err(e) => {
                error!("❌ Failed to create user: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Failed to create user")
                    .await;

                return Err(ServiceError::Repo(e));
            }
        };

        let assign_role_request = CreateUserRoleRequest {
            user_id: new_user.user_id,
            role_id: role.role_id,
        };

        if let Err(e) = self
            .user_role
            .assign_role_to_user(&assign_role_request)
            .await
        {
            error!(
                "❌ Failed to assign role {} to user {}: {:?}",
                role.role_id, new_user.user_id, e
            );
            self.tracing_metrics_core
                .complete_tracing_error(&tracing_ctx, method.clone(), "Failed to assign role")
                .await;
            return Err(ServiceError::Repo(e));
        }

        let user_response = UserResponse::from(new_user);

        info!(
            "✅ User registered successfully: {} {} ({})",
            user_response.firstname, user_response.lastname, user_response.email
        );

        self.tracing_metrics_core
            .complete_tracing_success(&tracing_ctx, method, "User registered successfully")
            .await;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: user_response,
        })
    }
    async fn login_user(
        &self,
        req: &AuthRequest,
    ) -> Result<ApiResponse<TokenResponse>, ServiceError> {
        let email = req.email.clone();

        info!("🔐 Incoming login request for user: {}", &req.email);

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "Login",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("user.email", email.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let failed_attempts_key = format!("auth:login_attempts:{email}");
        let current_attempts = self
            .cache_store
            .get_from_cache::<i32>(&failed_attempts_key)
            .await
            .unwrap_or(0);

        if current_attempts >= 5 {
            let msg = "Too many failed login attempts (rate limited)";
            warn!("❌ {}: {}", msg, email);
            self.tracing_metrics_core
                .complete_tracing_error(&tracing_ctx, method, msg)
                .await;
            return Err(ServiceError::Custom(
                "Too many failed attempts. Try again later.".to_string(),
            ));
        }

        let user = match self.query.find_by_email(email.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                error!("❌ User not found: {email}");

                let new_attempts = current_attempts + 1;

                self.cache_store
                    .set_to_cache(&failed_attempts_key, &new_attempts, Duration::minutes(15))
                    .await;

                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "User not found")
                    .await;
                return Err(ServiceError::Custom("user not found".to_string()));
            }
            Err(err) => {
                error!("❌ Failed to query user: {}", err);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Repo(err));
            }
        };

        if self
            .hashing
            .compare_password(&user.password, &req.password)
            .await
            .is_err()
        {
            error!("❌ Invalid password for user: {email}");

            let new_attempts = current_attempts + 1;

            self.cache_store
                .set_to_cache(&failed_attempts_key, &new_attempts, Duration::minutes(15))
                .await;

            self.tracing_metrics_core
                .complete_tracing_error(&tracing_ctx, method.clone(), "Invalid password")
                .await;
            return Err(ServiceError::InvalidCredentials);
        }

        self.cache_store
            .delete_from_cache(&failed_attempts_key)
            .await;

        let access_token = match self.token.create_access_token(user.user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate access token: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to generate access token",
                    )
                    .await;
                return Err(e);
            }
        };

        let refresh_token = match self.token.create_refresh_token(user.user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate refresh token: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to generate refresh token",
                    )
                    .await;
                return Err(e);
            }
        };

        let token = TokenResponse {
            access_token,
            refresh_token,
        };

        info!("✅ Login successful for email: {email}");

        self.tracing_metrics_core
            .complete_tracing_success(&tracing_ctx, method, "Login successful")
            .await;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Login successful".to_string(),
            data: token,
        })
    }
    async fn get_me(&self, id: i32) -> Result<ApiResponse<UserResponse>, ServiceError> {
        info!("📄 Fetching current user profile (get_me)");

        let method = Method::Get;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "GetMe",
            vec![
                KeyValue::new("component", "auth"),
                KeyValue::new("user.id", id.to_string()),
            ],
        );

        let cache_key = format!("auth:getme:{id}");

        if let Some(cached_user) = self
            .cache_store
            .get_from_cache::<UserResponse>(&cache_key)
            .await
        {
            info!("✅ Cache hit for user: {id}");
            self.tracing_metrics_core
                .complete_tracing_success(&tracing_ctx, method, "User fetched from cache")
                .await;
            return Ok(ApiResponse {
                status: "success".into(),
                message: "user fetched successfully (from cache)".into(),
                data: cached_user,
            });
        }

        let user = match self.query.find_by_id(id).await {
            Ok(user) => user,
            Err(e) => {
                error!("❌ Failed to fetch user from DB: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let user_response = UserResponse::from(user);

        self.cache_store
            .set_to_cache(&cache_key, &user_response, Duration::minutes(30))
            .await;

        self.tracing_metrics_core
            .complete_tracing_success(&tracing_ctx, method, "User profile fetched")
            .await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "user fetched successfully".into(),
            data: user_response,
        })
    }

    async fn refresh_token(&self, token: &str) -> Result<ApiResponse<TokenResponse>, ServiceError> {
        info!("🔄 Refreshing access token");

        let method = Method::Post;
        let tracing_ctx = self
            .tracing_metrics_core
            .start_tracing("RefreshToken", vec![KeyValue::new("component", "auth")]);

        let mut request = Request::new(token);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let user_id = match self.jwt_config.verify_token(token, "refresh") {
            Ok(uid) => uid,
            Err(ServiceError::TokenExpired) => {
                let _ = self.refresh_command.delete_token(token.to_string()).await;

                let _ = self
                    .cache_store
                    .delete_from_cache(&format!("auth:refresh:{token}"))
                    .await;

                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method, "Token expired")
                    .await;

                return Err(ServiceError::TokenExpired);
            }
            Err(e) => {
                error!("❌ Invalid token: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), "Invalid token")
                    .await;
                return Err(ServiceError::Custom("invalid token".to_string()));
            }
        };

        if let Err(e) = self.refresh_command.delete_token(token.to_string()).await {
            error!("❌ Failed to delete old refresh token: {:?}", e);
            self.tracing_metrics_core
                .complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    "Failed to delete old refresh token",
                )
                .await;
            return Err(ServiceError::from(e));
        }

        let access_token = match self.token.create_access_token(user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate access token: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to generate access token",
                    )
                    .await;
                return Err(e);
            }
        };

        let refresh_token = match self.token.create_refresh_token(user_id as i32).await {
            Ok(token) => token,
            Err(e) => {
                error!("❌ Failed to generate refresh token: {:?}", e);
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        "Failed to generate refresh token",
                    )
                    .await;
                return Err(e);
            }
        };

        let expiry = chrono::Utc::now() + chrono::Duration::hours(24);

        let update_req = &UpdateRefreshToken {
            user_id: user_id as i32,
            token: refresh_token.clone(),
            expires_at: expiry.naive_utc(),
        };

        if let Err(e) = self.refresh_command.update(update_req).await {
            error!("❌ Failed to update refresh token: {:?}", e);
            self.tracing_metrics_core
                .complete_tracing_error(&tracing_ctx, method, "Failed to update refresh token")
                .await;
            return Err(ServiceError::Custom(
                "Failed to update refresh token".into(),
            ));
        }

        self.cache_store
            .set_to_cache(
                &format!("auth:refresh:{refresh_token}"),
                &user_id,
                chrono::Duration::hours(24),
            )
            .await;

        self.tracing_metrics_core
            .complete_tracing_success(&tracing_ctx, method, "Token refreshed successfully")
            .await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "token refreshed".into(),
            data: TokenResponse {
                access_token,
                refresh_token,
            },
        })
    }
}
