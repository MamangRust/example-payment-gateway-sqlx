use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        topup::{
            repository::{command::DynTopupCommandRepository, query::DynTopupQueryRepository},
            service::command::TopupCommandServiceTrait,
        },
    },
    cache::CacheStore,
    domain::requests::{
        saldo::UpdateSaldoBalance,
        topup::{CreateTopupRequest, UpdateTopupAmount, UpdateTopupRequest, UpdateTopupStatus},
    },
    domain::responses::{ApiResponse, TopupResponse, TopupResponseDeleteAt},
    errors::{ServiceError, format_validation_errors},
    utils::{
        MetadataInjector, Method, Metrics, Status as StatusUtils, TracingContext, mask_card_number,
    },
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

pub struct TopupCommandService {
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub query: DynTopupQueryRepository,
    pub command: DynTopupCommandRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct TopupCommandServiceDeps {
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub query: DynTopupQueryRepository,
    pub command: DynTopupCommandRepository,
    pub cache_store: Arc<CacheStore>,
}

impl TopupCommandService {
    pub fn new(deps: TopupCommandServiceDeps) -> Result<Self> {
        let metrics = Metrics::new();

        let TopupCommandServiceDeps {
            card_query,
            saldo_query,
            saldo_command,
            query,
            command,
            cache_store,
        } = deps;

        Ok(Self {
            card_query,
            saldo_query,
            saldo_command,
            query,
            command,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("topup-command-service")
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
impl TopupCommandServiceTrait for TopupCommandService {
    async fn create(
        &self,
        req: &CreateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🚀 Starting CreateTopup: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_topup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "create"),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("topup_amount", req.topup_amount.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let masked_card = mask_card_number(&req.card_number);

        let _card = match self.card_query.find_by_card(&req.card_number).await {
            Ok(card) => card,
            Err(e) => {
                error!("❌ Failed to find card by number: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    "Database error while finding card",
                )
                .await;
                return Err(ServiceError::Custom("card not found".into()));
            }
        };

        let topup = match self.command.create(req).await {
            Ok(t) => t,
            Err(e) => {
                error!("❌ Failed to create topup: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Failed to create topup")
                    .await;
                return Err(ServiceError::Custom("failed to create topup".into()));
            }
        };

        let mut saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(s) => s,
            Err(e) => {
                error!("❌ Failed to find saldo: {e:?}");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id: topup.topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Saldo not found")
                    .await;
                return Err(ServiceError::Custom("saldo not found".into()));
            }
        };

        let new_balance = saldo.total_balance + req.topup_amount;
        saldo.total_balance = new_balance;

        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: req.card_number.clone(),
                total_balance: new_balance,
            })
            .await
        {
            error!("❌ Failed to update saldo: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id: topup.topup_id,
                    status: "failed".to_string(),
                })
                .await;
            self.complete_tracing_error(&tracing_ctx, method.clone(), "Failed to update saldo")
                .await;
            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        if let Err(e) = self
            .command
            .update_status(&UpdateTopupStatus {
                topup_id: topup.topup_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("❌ Failed to update topup status: {e:?}");
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                "Failed to update topup status",
            )
            .await;
            return Err(ServiceError::Custom("failed to update topup status".into()));
        }

        let response = TopupResponse::from(topup);

        info!(
            "✅ CreateTopup completed: card={} topup_amount={} new_balance={new_balance}",
            masked_card, req.topup_amount
        );

        self.complete_tracing_success(&tracing_ctx, method, "Topup created successfully")
            .await;

        let cache_keys = vec![
            format!("topup:find_all_by_card_number:card:{}:*", masked_card),
            format!("topup:find_by_card:card_number:{}", masked_card),
            "topup:find_by_id:*".to_string(),
            "topup:find_by_active:*".to_string(),
            "topup:find_by_trashed:*".to_string(),
            "topup:find_all:*".to_string(),
            format!("saldo:find_by_card:card_number:{}", masked_card),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup berhasil diproses".into(),
            data: response,
        })
    }
    async fn update(
        &self,
        req: &UpdateTopupRequest,
    ) -> Result<ApiResponse<TopupResponse>, ServiceError> {
        info!("🚀 Starting UpdateTopup: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let topup_id = req
            .topup_id
            .ok_or_else(|| ServiceError::Custom("topup_id is required".into()))?;

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_topup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "update"),
                KeyValue::new("topup_id", topup_id.to_string()),
                KeyValue::new("card_number", mask_card_number(&req.card_number)),
                KeyValue::new("topup_amount", req.topup_amount.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let masked_card = mask_card_number(&req.card_number);

        if let Err(e) = self.card_query.find_by_card(&req.card_number).await {
            error!("❌ Card not found: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id,
                    status: "failed".to_string(),
                })
                .await;
            self.complete_tracing_error(&tracing_ctx, method.clone(), "Card not found")
                .await;
            return Err(ServiceError::Custom("card not found".into()));
        }

        let existing = match self.query.find_by_id(topup_id).await {
            Ok(topup) => {
                info!("✅ Found topup with ID: {topup_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    "Topup retrieved successfully",
                )
                .await;
                topup
            }

            Err(e) => {
                error!("❌ Database error finding topup {topup_id}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom("Database error".into()));
            }
        };

        let difference = req.topup_amount - existing.topup_amount;

        if let Err(e) = self.command.update(req).await {
            error!("❌ Failed to update topup: {e:?}");
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id,
                    status: "failed".to_string(),
                })
                .await;
            self.complete_tracing_error(&tracing_ctx, method.clone(), "Failed to update topup")
                .await;
            return Err(ServiceError::Custom("failed to update topup".into()));
        }

        let mut saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(s) => s,
            Err(e) => {
                error!("❌ Failed to get saldo: {e:?}");
                let _ = self
                    .command
                    .update_status(&UpdateTopupStatus {
                        topup_id,
                        status: "failed".to_string(),
                    })
                    .await;
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Saldo not found")
                    .await;
                return Err(ServiceError::Custom("saldo not found".into()));
            }
        };

        let new_balance = saldo.total_balance + difference;
        saldo.total_balance = new_balance;

        if let Err(e) = self
            .saldo_command
            .update_balance(&UpdateSaldoBalance {
                card_number: req.card_number.clone(),
                total_balance: saldo.total_balance,
            })
            .await
        {
            error!("❌ Failed to update saldo: {e:?}");

            let _ = self
                .command
                .update_amount(&UpdateTopupAmount {
                    topup_id,
                    topup_amount: existing.topup_amount,
                })
                .await;
            let _ = self
                .command
                .update_status(&UpdateTopupStatus {
                    topup_id,
                    status: "failed".to_string(),
                })
                .await;
            self.complete_tracing_error(&tracing_ctx, method.clone(), "Failed to update saldo")
                .await;
            return Err(ServiceError::Custom("failed to update saldo".into()));
        }

        let updated_topup = match self.query.find_by_id(topup_id).await {
            Ok(topup) => {
                info!("✅ Found topup with ID: {topup_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method.clone(),
                    "Topup retrieved successfully",
                )
                .await;
                topup
            }

            Err(e) => {
                error!("❌ Failed to fetch updated topup {topup_id}: {e:?}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), "Database error")
                    .await;
                return Err(ServiceError::Custom(e.to_string()));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateTopupStatus {
                topup_id,
                status: "success".to_string(),
            })
            .await
        {
            error!("❌ Failed to update topup status: {e:?}");
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                "Failed to update topup status",
            )
            .await;
            return Err(ServiceError::Custom("failed to update topup status".into()));
        }

        let response = TopupResponse::from(updated_topup);

        info!(
            "✅ UpdateTopup completed: card={} topup_id={} new_amount={} new_balance={new_balance}",
            masked_card, topup_id, req.topup_amount
        );

        self.complete_tracing_success(&tracing_ctx, method, "Topup updated successfully")
            .await;

        let cache_keys = vec![
            format!("topup:find_by_id:id:{}", topup_id),
            format!("topup:find_by_card:card_number:{}", masked_card),
            format!("topup:find_all_by_card_number:card:{}:*", masked_card),
            "topup:find_all:*".to_string(),
            "topup:find_active:*".to_string(),
            format!("saldo:find_by_card:card_number:{}", masked_card),
            "saldo:find_all:*".to_string(),
            "saldo:find_active:*".to_string(),
            format!("card:find_by_card:number:{}", masked_card),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup berhasil diperbarui".into(),
            data: response,
        })
    }
    async fn trashed(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing topup id={topup_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_topup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut request = Request::new(topup_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let topup = match self.command.trashed(topup_id).await {
            Ok(topup) => {
                info!("✅ Topup trashed successfully: id={}", topup.topup_id);
                self.complete_tracing_success(&tracing_ctx, method, "Topup trashed successfully")
                    .await;
                topup
            }
            Err(e) => {
                error!("💥 Failed to trash topup id={topup_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash topup: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to trash topup with id {topup_id}"
                )));
            }
        };

        let response = TopupResponseDeleteAt::from(topup);

        let cache_keys = vec![
            format!("topup:find_by_id:id:{}", topup_id),
            format!("topup:find_by_card:card_number:{}", response.card_number),
            "topup:find_active:*".to_string(),
            "topup:find_all:*".to_string(),
            "topup:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup trashed successfully".into(),
            data: response,
        })
    }

    async fn restore(
        &self,
        topup_id: i32,
    ) -> Result<ApiResponse<TopupResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring topup id={topup_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_topup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut request = Request::new(topup_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        let topup = match self.command.restore(topup_id).await {
            Ok(topup) => {
                info!("✅ Topup restored successfully: id={}", topup.topup_id);
                self.complete_tracing_success(&tracing_ctx, method, "Topup restored successfully")
                    .await;
                topup
            }
            Err(e) => {
                error!("💥 Failed to restore topup id={topup_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore topup: {:?}", e),
                )
                .await;
                return Err(ServiceError::Custom(format!(
                    "Failed to restore topup with id {topup_id}"
                )));
            }
        };

        let response = TopupResponseDeleteAt::from(topup);

        let cache_keys = vec![
            format!("topup:find_by_id:id:{}", topup_id),
            format!("topup:find_by_card:card_number:{}", response.card_number),
            "topup:find_active:*".to_string(),
            "topup:find_all:*".to_string(),
            "topup:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Topup restored successfully".into(),
            data: response,
        })
    }

    async fn delete_permanent(&self, topup_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting topup id={topup_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_topup",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("topup_id", topup_id.to_string()),
            ],
        );

        let mut request = Request::new(topup_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(topup_id).await {
            Ok(_) => {
                info!("✅ Topup permanently deleted: id={topup_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Topup permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    format!("topup:find_by_id:id:{}", topup_id),
                    "topup:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Topup permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete topup id={topup_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete topup: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete topup with id {topup_id}"
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring all trashed topups");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_topups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All topups restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All topups restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "topup:find_active:*".to_string(),
                    "topup:find_all:*".to_string(),
                    "topup:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All topups restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all topups: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all topups: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to restore all trashed topups".into(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting all trashed topups");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_topups",
            vec![
                KeyValue::new("component", "topup"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All topups permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All topups permanently deleted successfully",
                )
                .await;

                let cache_keys = vec![
                    "topup:find_trashed:*",
                    "topup:find_active:*",
                    "topup:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All topups permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to delete all topups: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to delete all topups: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed topups".into(),
                ))
            }
        }
    }
}
