use crate::utils::mask_api_key;
use crate::{
    abstract_trait::{
        merchant::{
            repository::command::DynMerchantCommandRepository,
            service::command::MerchantCommandServiceTrait,
        },
        user::repository::query::DynUserQueryRepository,
    },
    cache::CacheStore,
    domain::{
        requests::merchant::{CreateMerchantRequest, UpdateMerchantRequest, UpdateMerchantStatus},
        responses::{ApiResponse, MerchantResponse, MerchantResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, generate_api_key,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
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

pub struct MerchantCommandService {
    pub command: DynMerchantCommandRepository,
    pub user_query: DynUserQueryRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

impl MerchantCommandService {
    pub fn new(
        command: DynMerchantCommandRepository,
        user_query: DynUserQueryRepository,
        cache_store: Arc<CacheStore>,
    ) -> Result<Self> {
        let metrics = Metrics::new();

        Ok(Self {
            command,
            user_query,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("merchant-command-service")
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
impl MerchantCommandServiceTrait for MerchantCommandService {
    async fn create(
        &self,
        req: &CreateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!(
            "🆕 Creating merchant: {} for user_id={}",
            req.name, req.user_id
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "create"),
                KeyValue::new("merchant.name", req.name.clone()),
                KeyValue::new("merchant.user_id", req.user_id.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let api_key = generate_api_key();

        let _user = match self.user_query.find_by_id(req.user_id).await {
            Ok(user) => {
                info!("👤 Found user with id {}", req.user_id);

                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Successfully fetched user with id {}", req.user_id),
                )
                .await;

                user
            }

            Err(e) => {
                error!("👤 Failed to fetch user with id {}: {:?}", req.user_id, e);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error when fetching user {}: {}", req.user_id, e),
                )
                .await;

                return Err(ServiceError::Custom("Failed to fetch user".into()));
            }
        };

        let merchant = match self.command.create(api_key, req).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant created successfully: id={}",
                    merchant.merchant_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant created successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                let error_msg = format!(
                    "💥 Failed to create merchant {} (user_id={}): {e:?}",
                    req.name, req.user_id
                );
                error!("{}", error_msg);
                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg));
            }
        };

        let response = MerchantResponse::from(merchant);

        let masked_key = mask_api_key(&response.api_key);

        let cache_key = [
            format!("merchant:find_by_apikey:key:{masked_key}"),
            format!("merchant:find_by_id:id:{}", response.id),
            format!("merchant:find_by_user_id:user_id:{}", response.user_id),
        ];

        for key in cache_key {
            self.cache_store
                .set_to_cache(&key, &response, Duration::minutes(10))
                .await;
        }

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant created successfully".to_string(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateMerchantRequest,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let merchant_id = req
            .merchant_id
            .ok_or_else(|| ServiceError::Custom("merchant_id is required".into()))?;

        info!("🔄 Updating merchant id={merchant_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "update"),
                KeyValue::new("merchant.id", merchant_id.to_string()),
                KeyValue::new("merchant.user_id", req.user_id.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let _user = match self.user_query.find_by_id(req.user_id).await {
            Ok(user) => {
                info!("👤 Found user with id {}", req.user_id);

                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Successfully fetched user with id {}", req.user_id),
                )
                .await;

                user
            }

            Err(e) => {
                error!("👤 Failed to fetch user with id {}: {:?}", req.user_id, e);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error when fetching user {}: {}", req.user_id, e),
                )
                .await;

                return Err(ServiceError::Custom("Failed to fetch user".into()));
            }
        };

        let updated_merchant = match self.command.update(req).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant updated successfully: id={}",
                    merchant.merchant_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant updated successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                let error_msg = format!("💥 Failed to update merchant id={merchant_id}: {e:?}");
                error!("{error_msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg));
            }
        };

        let response = MerchantResponse::from(updated_merchant);
        let masked_key = mask_api_key(&response.api_key);

        let cache_keys = vec![
            format!("merchant:find_by_id:id:{}", response.id),
            format!("merchant:find_by_user_id:user_id:{}", response.user_id),
            format!("merchant:find_by_apikey:key:{masked_key}"),
            "merchant:find_all:*".to_string(),
            "merchant:find_active:*".to_string(),
            "merchant:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant updated successfully".to_string(),
            data: response,
        })
    }

    async fn update_status(
        &self,
        req: &UpdateMerchantStatus,
    ) -> Result<ApiResponse<MerchantResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let merchant_id = req
            .merchant_id
            .ok_or_else(|| ServiceError::Custom("merchant_id is required".into()))?;

        info!(
            "🔄 Updating status for merchant id={merchant_id} to {}",
            req.status
        );

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_merchant_status",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "update_status"),
                KeyValue::new("merchant.id", merchant_id.to_string()),
                KeyValue::new("merchant.status", req.status.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let updated_merchant = match self.command.update_status(req).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant status updated successfully: id={}, status={}",
                    merchant.merchant_id, merchant.status
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant status updated successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                let error_msg = format!(
                    "💥 Failed to update status for merchant id={merchant_id} to {}: {e:?}",
                    req.status
                );
                error!("{error_msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg));
            }
        };

        let response = MerchantResponse::from(updated_merchant);

        let masked_key = mask_api_key(&response.api_key);

        let cache_keys = vec![
            format!("merchant:find_by_id:id:{}", response.id),
            format!("merchant:find_by_user_id:user_id:{}", response.user_id),
            format!("merchant:find_by_apikey:key:{masked_key}"),
            "merchant:find_all:*".to_string(),
            "merchant:find_active:*".to_string(),
            "merchant:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Merchant status updated successfully".to_string(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing merchant id={id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("merchant.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let merchant = match self.command.trash(id).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant trashed successfully: id={}",
                    merchant.merchant_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant trashed successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                error!("💥 Failed to trash merchant id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash merchant: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to trash merchant".into()));
            }
        };

        let response = MerchantResponseDeleteAt::from(merchant);
        let masked_key = mask_api_key(&response.api_key);

        let cache_keys = vec![
            format!("merchant:find_by_id:id:{id}"),
            format!("merchant:find_by_user_id:user_id:{}", response.user_id),
            format!("merchant:find_by_apikey:key:{masked_key}"),
            "merchant:find_all:*".to_string(),
            "merchant:find_active:*".to_string(),
            "merchant:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Merchant trashed successfully".into(),
            data: response,
        })
    }

    async fn restore(
        &self,
        id: i32,
    ) -> Result<ApiResponse<MerchantResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring merchant id={id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("merchant.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let merchant = match self.command.restore(id).await {
            Ok(merchant) => {
                info!(
                    "✅ Merchant restored successfully: id={}",
                    merchant.merchant_id
                );
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant restored successfully",
                )
                .await;
                merchant
            }
            Err(e) => {
                error!("💥 Failed to restore merchant id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore merchant: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to restore merchant".into()));
            }
        };

        let response = MerchantResponseDeleteAt::from(merchant);

        let masked_key = mask_api_key(&response.api_key);

        let cache_keys = vec![
            format!("merchant:find_by_id:id:{id}"),
            format!("merchant:find_by_user_id:user_id:{}", response.user_id),
            format!("merchant:find_by_apikey:key:{masked_key}"),
            "merchant:find_all:*".to_string(),
            "merchant:find_active:*".to_string(),
            "merchant:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Merchant restored successfully".into(),
            data: response,
        })
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting merchant id={id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_merchant",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("merchant.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(id).await {
            Ok(_) => {
                info!("✅ Merchant permanently deleted: id={id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Merchant permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    format!("merchant:find_by_id:id:{id}"),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_active:*".to_string(),
                    "merchant:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: format!("Merchant with id={id} permanently deleted"),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete merchant id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete merchant: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete merchant".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed merchants");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_merchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All merchants restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All merchants restored successfully",
                )
                .await;

                let cache_keys = vec![
                    format!("merchant:find_by_id:id:*"),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_active:*".to_string(),
                    "merchant:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All trashed merchants restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all merchants: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all merchants: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to restore all merchants".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed merchants");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_merchants",
            vec![
                KeyValue::new("component", "merchant"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All merchants permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All merchants permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    "merchant:find_by_id:id:*".to_string(),
                    "merchant:find_by_user_id:user_id:*".to_string(),
                    "merchant:find_by_apikey:key:*".to_string(),
                    "merchant:find_all:*".to_string(),
                    "merchant:find_active:*".to_string(),
                    "merchant:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All trashed merchants permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to delete all merchants: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to delete all merchants: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to delete all merchants".into(),
                ))
            }
        }
    }
}
