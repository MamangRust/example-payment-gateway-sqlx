use crate::{
    abstract_trait::{
        card::{
            repository::{command::DynCardCommandRepository, query::DynCardQueryRepository},
            service::command::CardCommandServiceTrait,
        },
        user::repository::query::DynUserQueryRepository,
    },
    cache::CacheStore,
    domain::{
        requests::card::{CreateCardRequest, UpdateCardRequest},
        responses::{ApiResponse, CardResponse, CardResponseDeleteAt},
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

pub struct CardCommandService {
    pub user_query: DynUserQueryRepository,
    pub query: DynCardQueryRepository,
    pub command: DynCardCommandRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct CardCommandServiceDeps {
    pub user_query: DynUserQueryRepository,
    pub query: DynCardQueryRepository,
    pub command: DynCardCommandRepository,
    pub cache_store: Arc<CacheStore>,
}

impl CardCommandService {
    pub fn new(deps: CardCommandServiceDeps) -> Result<Self> {
        let metrics = Metrics::new();

        let CardCommandServiceDeps {
            user_query,
            query,
            command,
            cache_store,
        } = deps;

        Ok(Self {
            user_query,
            query,
            command,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("card-command-service")
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
impl CardCommandServiceTrait for CardCommandService {
    async fn create(
        &self,
        req: &CreateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating card for user_id={}", req.user_id);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "CreateCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "create"),
                KeyValue::new("card.user_id", req.user_id.to_string()),
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

        let card = match self.command.create(req).await {
            Ok(card) => {
                info!("✅ Card created successfully with card_id={}", card.card_id);
                self.complete_tracing_success(&tracing_ctx, method, "Card created successfully")
                    .await;
                card
            }
            Err(e) => {
                error!(
                    "💥 Failed to create card for user_id {}: {e:?}",
                    req.user_id,
                );
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to create card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to create card".into()));
            }
        };

        let response = CardResponse::from(card);

        let cache_key = format!("card:find_by_user_id:user_id:{}", req.user_id);
        self.cache_store.delete_from_cache(&cache_key).await;

        info!("✅ Card created successfully with card_id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Card created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateCardRequest,
    ) -> Result<ApiResponse<CardResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let card_id = req
            .card_id
            .ok_or_else(|| ServiceError::Custom("card_id is required".into()))?;

        info!("🔄 Updating card id={card_id} for user_id={}", req.user_id);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "UpdateCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "update"),
                KeyValue::new("card.card_id", card_id.to_string()),
                KeyValue::new("card.user_id", req.user_id.to_string()),
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

        let updated_card = match self.command.update(req).await {
            Ok(card) => {
                info!("✅ Card updated successfully with card_id={}", card.card_id);
                self.complete_tracing_success(&tracing_ctx, method, "Card updated successfully")
                    .await;
                card
            }
            Err(e) => {
                error!("💥 Failed to update card id {card_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to update card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to update card".into()));
            }
        };

        let response = CardResponse::from(updated_card);

        let cache_keys = vec![
            format!("card:find_by_id:id:{}", card_id),
            format!("card:find_by_user_id:user_id:{}", req.user_id),
            format!("card:find_by_card:number:{}", response.card_number),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        info!("✅ Card updated successfully with card_id={}", response.id);

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Card updated successfully!".into(),
            data: response,
        })
    }
    async fn trash(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing card id={id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "TrashCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("card.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let card = match self.command.trash(id).await {
            Ok(card) => {
                info!("✅ Card trashed successfully with card_id={}", card.card_id);
                self.complete_tracing_success(&tracing_ctx, method, "Card trashed successfully")
                    .await;
                card
            }
            Err(e) => {
                error!("💥 Failed to trash card id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to trash card".into()));
            }
        };

        let response = CardResponseDeleteAt::from(card);

        let cache_keys = vec![
            format!("card:find_by_id:id:{}", id),
            format!("card:find_by_user_id:user_id:{}", response.user_id),
            format!("card:find_by_card:number:{}", response.card_number),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🗑️ Card trashed successfully!".into(),
            data: response,
        })
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<CardResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring card id={id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("card.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let card = match self.command.restore(id).await {
            Ok(card) => {
                info!(
                    "✅ Card restored successfully with card_id={}",
                    card.card_id
                );
                self.complete_tracing_success(&tracing_ctx, method, "Card restored successfully")
                    .await;
                card
            }
            Err(e) => {
                error!("💥 Failed to restore card id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore card: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom("Failed to restore card".into()));
            }
        };

        let response = CardResponseDeleteAt::from(card);

        let cache_keys = vec![
            format!("card:find_by_id:id:{id}"),
            format!("card:find_by_user_id:user_id:{}", response.user_id),
            format!("card:find_by_card:number:{}", response.card_number),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "♻️ Card restored successfully!".into(),
            data: response,
        })
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting card id={id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "DeleteCard",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("card.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let card = match self.query.find_by_id(id).await {
            Ok(card) => {
                info!("📇 Successfully fetched card with id {id}");

                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Successfully fetched card with id {id}"),
                )
                .await;

                card
            }

            Err(e) => {
                error!("💥 Failed to fetch card with id {id}: {:?}", e);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Database error when fetching card {id}: {:?}", e),
                )
                .await;

                return Err(ServiceError::Custom("Failed to fetch card".into()));
            }
        };

        match self.command.delete_permanent(id).await {
            Ok(_) => {
                info!("✅ Card permanently deleted with card_id={id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Card permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    format!("card:find_by_id:id:{id}"),
                    format!("card:find_by_card:number:{}", card.card_number),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🧨 Card permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete card id={id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete card: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete card".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed cards");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "RestoreAllCards",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All trashed cards restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All trashed cards restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "card:find_trashed:*",
                    "card:find_active:*",
                    "card:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🔄 All cards restored successfully!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all cards: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all cards: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom("Failed to restore all cards".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed cards");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "DeleteAllCards",
            vec![
                KeyValue::new("component", "card"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All trashed cards permanently deleted successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All trashed cards permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    "card:find_trashed:*",
                    "card:find_active:*",
                    "card:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "💣 All cards permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to delete all cards: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to delete all cards: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom("Failed to delete all cards".into()))
            }
        }
    }
}
