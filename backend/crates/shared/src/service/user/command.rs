use crate::{
    abstract_trait::{
        hashing::DynHashing,
        role::repository::query::DynRoleQueryRepository,
        user::{
            repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
            service::command::UserCommandServiceTrait,
        },
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    domain::{
        requests::{
            user::{CreateUserRequest, UpdateUserRequest},
            user_role::CreateUserRoleRequest,
        },
        responses::{ApiResponse, UserResponse, UserResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
    utils::{MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext},
};
use anyhow::Result;
use async_trait::async_trait;
use opentelemetry::{
    Context, KeyValue,
    global::{self, BoxedTracer},
    trace::{Span, SpanKind, TraceContextExt, Tracer},
};
use std::sync::Arc;
use tokio::time::Instant;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct UserCommandService {
    pub query: DynUserQueryRepository,
    pub command: DynUserCommandRepository,
    pub hashing: DynHashing,
    pub user_role: DynUserRoleCommandRepository,
    pub role: DynRoleQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct UserCommandServiceDeps {
    pub query: DynUserQueryRepository,
    pub command: DynUserCommandRepository,
    pub hashing: DynHashing,
    pub user_role: DynUserRoleCommandRepository,
    pub role: DynRoleQueryRepository,
    pub cache_store: Arc<CacheStore>,
}

impl UserCommandService {
    pub fn new(deps: UserCommandServiceDeps) -> Result<Self> {
        let UserCommandServiceDeps {
            query,
            command,
            hashing,
            user_role,
            role,
            cache_store,
        } = deps;

        let metrics = Metrics::new();

        Ok(Self {
            query,
            command,
            hashing,
            user_role,
            role,
            metrics,
            cache_store,
        })
    }

    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("user-command-service")
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
            info!("✅ Operation completed successfully: {message}");
        } else {
            error!("❌ Operation failed: {message}");
        }

        self.metrics.record(method, status, elapsed);

        tracing_ctx.cx.span().end();
    }
}

#[async_trait]
impl UserCommandServiceTrait for UserCommandService {
    async fn create(
        &self,
        req: &CreateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating user: {} {}", req.firstname, req.lastname);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_user",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "create"),
                KeyValue::new("email", req.email.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let existing_email = match self.query.find_by_email(req.email.clone()).await {
            Ok(opt) => {
                info!("Checked existing email {}", req.email);
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Checked existing email {}", req.email),
                )
                .await;
                opt
            }
            Err(e) => {
                let msg = format!("💥 Failed to check existing email {}: {:?}", req.email, e);
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        if existing_email.is_some() {
            let msg = format!("📧 Email {} already registered", req.email);
            error!("{msg}");
            self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                .await;
            return Err(ServiceError::Custom(msg));
        }

        let hashed_password = match self.hashing.hash_password(&req.password).await {
            Ok(hash) => hash,
            Err(e) => {
                let msg = format!("❌ Failed to hash password: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
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
                let msg = format!("❌ Role not found: {}", DEFAULT_ROLE_NAME);
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom("Default role not found".to_string()));
            }
            Err(e) => {
                let msg = format!("❌ Failed to query role: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Repo(e));
            }
        };

        let new_request = &CreateUserRequest {
            firstname: req.firstname.clone(),
            lastname: req.lastname.clone(),
            password: hashed_password,
            email: req.email.clone(),
            confirm_password: req.confirm_password.clone(),
        };

        let new_user = match self.command.create(new_request).await {
            Ok(user) => {
                let msg = format!("✅ User created successfully: {}", user.email);
                info!("{msg}");
                user
            }
            Err(e) => {
                let msg = format!("💥 Failed to create user {}: {e:?}", req.email);
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
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
            let msg = format!(
                "❌ Failed to assign role {} to user {}: {e:?}",
                role.role_id, new_user.user_id,
            );
            error!("{msg}");
            self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                .await;
            return Err(ServiceError::Repo(e));
        }

        let response = UserResponse::from(new_user);

        self.complete_tracing_success(&tracing_ctx, method, "User created successfully")
            .await;

        let cache_keys = vec![
            "user:find_all:*",
            "user:find_by_active:*",
            "user:find_by_trashed",
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(key).await;
        }

        info!("✅ User created successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ User created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateUserRequest,
    ) -> Result<ApiResponse<UserResponse>, ServiceError> {
        req.validate().map_err(|e| {
            let msg = format!("❌ Validation failed: {e:?}");
            error!("{msg}");
            ServiceError::Custom(msg)
        })?;

        let user_id = req
            .id
            .ok_or_else(|| ServiceError::Custom("user_id is required".into()))?;
        info!("🔄 Updating user id={user_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_user",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "update"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let user = match self.query.find_by_id(user_id).await {
            Ok(user) => user,
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        if let Some(new_email) = &req.email {
            let new_email_norm = new_email.trim().to_lowercase();
            let old_email_norm = user.email.trim().to_lowercase();

            if new_email_norm == old_email_norm {
                info!("📧 Email unchanged after normalization, skipping DB check");
            } else {
                match self.query.find_by_email(new_email_norm.clone()).await {
                    Ok(Some(_)) => {
                        let msg = format!("📧 Email {new_email} already used by another user");
                        error!("{msg}");
                        self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                            .await;
                        return Err(ServiceError::Custom(msg));
                    }
                    Ok(None) => {
                        info!("📧 Email {new_email} available for use");
                    }
                    Err(e) => {
                        let msg = format!("💥 Failed to check email {new_email}: {e:?}");
                        error!("{msg}");
                        self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                            .await;
                        return Err(ServiceError::Custom(msg));
                    }
                }
            }
        } else {
            info!("📧 No email in request, skip email validation");
        }

        let updated_user = match self.command.update(req).await {
            Ok(user) => {
                let msg = format!("✅ User updated successfully: {}", user.email);
                info!("{msg}");
                user
            }
            Err(e) => {
                let msg = format!("💥 Failed to update user {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let response = UserResponse::from(updated_user);

        self.complete_tracing_success(&tracing_ctx, method, "User updated successfully")
            .await;

        let cache_pattern = format!("user:find_by_id:id:{}", user_id);

        self.cache_store.delete_from_cache(&cache_pattern).await;

        info!("✅ User updated successfully with id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ User updated successfully!".into(),
            data: response,
        })
    }

    async fn trashed(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing user id={}", user_id);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_user",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let _user = match self.query.find_by_id(user_id).await {
            Ok(user) => user,
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let trashed_user = match self.command.trashed(user_id).await {
            Ok(user) => {
                let msg = format!("✅ User trashed successfully: {}", user.email);
                info!("{msg}");
                user
            }
            Err(e) => {
                let msg = format!("💥 Failed to trash user {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let response = UserResponseDeleteAt::from(trashed_user);

        self.complete_tracing_success(&tracing_ctx, method, "User trashed successfully")
            .await;

        let cache_keys = vec![
            format!("user:find_by_id:id:{}", user_id),
            "user:find_all:*".to_string(),
            "user:find_active:*".to_string(),
            "user:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🗑️ User trashed successfully!".into(),
            data: response,
        })
    }

    async fn restore(
        &self,
        user_id: i32,
    ) -> Result<ApiResponse<UserResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring user id={user_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_user",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let _user = match self.query.find_by_id(user_id).await {
            Ok(user) => user,
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let restored_user = match self.command.restore(user_id).await {
            Ok(user) => {
                let msg = format!("✅ User restored successfully: {}", user.email);
                info!("{msg}");
                user
            }
            Err(e) => {
                let msg = format!("💥 Failed to restore user {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        let response = UserResponseDeleteAt::from(restored_user);

        self.complete_tracing_success(&tracing_ctx, method, "User restored successfully")
            .await;

        let cache_keys = vec![
            format!("user:find_by_id:id:{}", user_id),
            "user:find_all:*".to_string(),
            "user:find_active:*".to_string(),
            "user:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "♻️ User restored successfully!".into(),
            data: response,
        })
    }

    async fn delete_permanent(&self, user_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting user id={user_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_permanent_user",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("user_id", user_id.to_string()),
            ],
        );

        let mut request = Request::new(user_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let user = match self.query.find_by_id(user_id).await {
            Ok(user) => user,
            Err(e) => {
                let msg = format!("👤 User not found with id {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                return Err(ServiceError::Custom(msg));
            }
        };

        match self.command.delete_permanent(user_id).await {
            Ok(_) => {
                let msg = format!("✅ User permanently deleted: {}", user.email);
                info!("{msg}");
                self.complete_tracing_success(&tracing_ctx, method, "User permanently deleted")
                    .await;

                let cache_keys = vec![
                    format!("user:find_by_id:id:{}", user_id),
                    "user:find_all:*".to_string(),
                    "user:find_active:*".to_string(),
                    "user:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🧨 User permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                let msg = format!("💥 Failed to permanently delete user {user_id}: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed users");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_users",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All users restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All users restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "user:find_all:*".to_string(),
                    "user:find_active:*".to_string(),
                    "user:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🔄 All users restored successfully!".into(),
                    data: true,
                })
            }
            Err(e) => {
                let msg = format!("💥 Failed to restore all users: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                Err(ServiceError::Custom(msg))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed users");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_users",
            vec![
                KeyValue::new("component", "user"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All users permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All users permanently deleted",
                )
                .await;

                let cache_keys = vec![
                    "user:find_all:*".to_string(),
                    "user:find_active:*".to_string(),
                    "user:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "💣 All users permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                let msg = format!("💥 Failed to permanently delete all users: {e:?}");
                error!("{msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &msg)
                    .await;
                Err(ServiceError::Custom(msg))
            }
        }
    }
}
