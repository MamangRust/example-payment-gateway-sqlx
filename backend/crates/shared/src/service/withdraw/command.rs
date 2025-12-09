use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::repository::{command::DynSaldoCommandRepository, query::DynSaldoQueryRepository},
        withdraw::{
            repository::{
                command::DynWithdrawCommandRepository, query::DynWithdrawQueryRepository,
            },
            service::command::WithdrawCommandServiceTrait,
        },
    },
    cache::CacheStore,
    domain::{
        requests::{
            saldo::UpdateSaldoWithdraw,
            withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest, UpdateWithdrawStatus},
        },
        responses::{ApiResponse, WithdrawResponse, WithdrawResponseDeleteAt},
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

pub struct WithdrawCommandService {
    pub query: DynWithdrawQueryRepository,
    pub command: DynWithdrawCommandRepository,
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub metrics: Metrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct WithdrawCommandServiceDeps {
    pub query: DynWithdrawQueryRepository,
    pub command: DynWithdrawCommandRepository,
    pub card_query: DynCardQueryRepository,
    pub saldo_query: DynSaldoQueryRepository,
    pub saldo_command: DynSaldoCommandRepository,
    pub cache_store: Arc<CacheStore>,
}

impl WithdrawCommandService {
    pub fn new(deps: WithdrawCommandServiceDeps) -> Result<Self> {
        let metrics = Metrics::new();

        let WithdrawCommandServiceDeps {
            query,
            command,
            card_query,
            saldo_query,
            saldo_command,
            cache_store,
        } = deps;

        Ok(Self {
            query,
            command,
            card_query,
            saldo_query,
            saldo_command,
            metrics,
            cache_store,
        })
    }
    fn get_tracer(&self) -> BoxedTracer {
        global::tracer("withdraw-command-service")
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
impl WithdrawCommandServiceTrait for WithdrawCommandService {
    async fn create(
        &self,
        req: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("creating new withdraw: {:?}", req);

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "create_withdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "create"),
                KeyValue::new("withdraw.amount", req.withdraw_amount.to_string()),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        let _card = match self.card_query.find_by_card(&req.card_number).await {
            Ok(card) => card,

            Err(e) => {
                error!("❌ failed to find card {}: {e:?}", req.card_number);

                let error_msg = format!("failed to find card {}", req.card_number);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.to_string()));
            }
        };

        let saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(saldo) => saldo,

            Err(e) => {
                error!(
                    "❌ failed to find saldo for card {}: {e:?}",
                    req.card_number
                );

                let error_msg = format!("failed to find saldo for card {}", req.card_number);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg));
            }
        };

        if saldo.total_balance < req.withdraw_amount {
            let error_msg = format!(
                "error insufficient balance, requested: {}, available: {}",
                req.withdraw_amount, saldo.total_balance
            );
            error!("{error_msg}");
            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        let new_total_balance = saldo.total_balance - req.withdraw_amount;

        let update_data = UpdateSaldoWithdraw {
            card_number: req.card_number.clone(),
            total_balance: new_total_balance as i32,
            withdraw_amount: req.withdraw_amount as i32,
            withdraw_time: req.withdraw_time,
        };

        let _ = match self.saldo_command.update_withdraw(&update_data).await {
            Ok(updated) => updated,

            Err(e) => {
                error!("❌ failed to update saldo: {e:?}");

                let error_msg = "failed to update saldo";

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let withdraw_record = match self.command.create(req).await {
            Ok(record) => {
                info!("created withdraw record {:?}", record.withdraw_id);
                record
            }
            Err(e) => {
                let error_msg = format!("failed to create withdraw record: {:?}", e);
                error!("{error_msg}");

                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

                let rollback_data = UpdateSaldoWithdraw {
                    card_number: req.card_number.clone(),
                    total_balance: saldo.total_balance as i32,
                    withdraw_amount: req.withdraw_amount as i32,
                    withdraw_time: req.withdraw_time,
                };

                if let Err(rollback_err) = self.saldo_command.update_withdraw(&rollback_data).await
                {
                    error!("error rollback {rollback_err:?}");
                }

                return Err(ServiceError::Custom(
                    "failed to create withdraw record".into(),
                ));
            }
        };

        let update_status = UpdateWithdrawStatus {
            withdraw_id: withdraw_record.withdraw_id,
            status: "success".to_string(),
        };

        if let Err(e) = self.command.update_status(&update_status).await {
            let error_msg = format!("failed to update withdraw status: {:?}", e);
            error!("{error_msg}");

            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;

            let update_status_failed = UpdateWithdrawStatus {
                withdraw_id: withdraw_record.withdraw_id,
                status: "failed".to_string(),
            };

            if let Err(e2) = self.command.update_status(&update_status_failed).await {
                error!("error {e2:?}");
            }

            return Err(ServiceError::Custom(
                "failed to update withdraw status".into(),
            ));
        }

        info!("success withdraw {:?}", withdraw_record.withdraw_id);

        let withdraw_response = WithdrawResponse::from(withdraw_record);

        let cache_keys = vec![
            format!("withdraw:find_by_card:{}", req.card_number),
            format!("saldo:find_by_card:{}", req.card_number),
            format!("withdraw:find_by_id:{}", withdraw_response.clone().id),
            "withdraw:find_all:*".to_string(),
            "withdraw:find_by_active:*".to_string(),
            "withdraw:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }
        self.complete_tracing_success(&tracing_ctx, method, "Withdraw created successfully")
            .await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "created withdraw successfully".into(),
            data: withdraw_response,
        })
    }
    async fn update(
        &self,
        req: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ServiceError> {
        info!("updating withdraw: {:?}", req);

        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "update_withdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "update"),
                KeyValue::new(
                    "withdraw.id",
                    req.withdraw_id.unwrap_or_default().to_string(),
                ),
                KeyValue::new("withdraw.amount", req.withdraw_amount.to_string()),
            ],
        );

        let mut request_with_trace = Request::new(req.clone());
        self.inject_trace_context(&tracing_ctx.cx, &mut request_with_trace);

        let withdraw_id = match req.withdraw_id {
            Some(id) => id,
            None => {
                let error_msg = "withdraw_id is required";

                self.complete_tracing_error(&tracing_ctx, method.clone(), error_msg)
                    .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let _card = match self.card_query.find_by_card(&req.card_number).await {
            Ok(card) => card,

            Err(e) => {
                error!("❌ failed to find card: {e:?}");
                let error_msg = "failed to find card";

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg.into()));
            }
        };

        let _withdraw = match self.query.find_by_id(withdraw_id).await {
            Ok(w) => w,

            Err(e) => {
                error!("❌ failed to find withdraw {}: {e:?}", withdraw_id);

                let error_msg = format!("failed to find withdraw {}", withdraw_id);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg));
            }
        };

        let saldo = match self.saldo_query.find_by_card(&req.card_number).await {
            Ok(s) => s,

            Err(e) => {
                error!(
                    "❌ failed to fetch saldo for card {}: {e:?}",
                    req.card_number
                );

                let error_msg = format!("failed to fetch saldo for card {}", req.card_number);

                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("{}: {:?}", error_msg, e),
                )
                .await;

                return Err(ServiceError::Custom(error_msg));
            }
        };

        if saldo.total_balance < req.withdraw_amount {
            let error_msg = format!(
                "error insufficient balance, requested: {}, available: {}",
                req.withdraw_amount, saldo.total_balance
            );
            error!("{error_msg}");
            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;
            return Err(ServiceError::Custom("insufficient balance".into()));
        }

        let new_total_balance = saldo.total_balance - req.withdraw_amount;

        let update_saldo_data = UpdateSaldoWithdraw {
            card_number: req.card_number.clone(),
            total_balance: new_total_balance as i32,
            withdraw_amount: req.withdraw_amount as i32,
            withdraw_time: req.withdraw_time,
        };

        if let Err(e) = self.saldo_command.update_withdraw(&update_saldo_data).await {
            error!("error {e:?}");
            let error_msg = "failed to update saldo";
            self.complete_tracing_error(
                &tracing_ctx,
                method.clone(),
                &format!("{}: {:?}", error_msg, e),
            )
            .await;

            if let Err(e2) = self
                .command
                .update_status(&UpdateWithdrawStatus {
                    withdraw_id,
                    status: "failed".to_string(),
                })
                .await
            {
                error!("error {e2:?}");
            }

            return Err(ServiceError::Custom(error_msg.to_string()));
        }

        let updated_withdraw = match self.command.update(req).await {
            Ok(record) => record,
            Err(e) => {
                let error_msg = format!("failed to update withdraw record: {:?}", e);
                error!("{error_msg}");
                self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

                let rollback_data = UpdateSaldoWithdraw {
                    card_number: req.card_number.clone(),
                    total_balance: saldo.total_balance as i32,
                    withdraw_amount: req.withdraw_amount as i32,
                    withdraw_time: req.withdraw_time,
                };
                if let Err(rollback_err) = self.saldo_command.update_withdraw(&rollback_data).await
                {
                    error!("error rollback {rollback_err:?}");
                }

                let _ = self
                    .command
                    .update_status(&UpdateWithdrawStatus {
                        withdraw_id,
                        status: "failed".to_string(),
                    })
                    .await
                    .map_err(|e2| {
                        error!("error {e2:?}");
                        e2
                    });

                return Err(ServiceError::Custom(
                    "failed to update withdraw record".into(),
                ));
            }
        };

        if let Err(e) = self
            .command
            .update_status(&UpdateWithdrawStatus {
                withdraw_id: updated_withdraw.withdraw_id,
                status: "success".to_string(),
            })
            .await
        {
            let error_msg = format!("failed to update withdraw status: {:?}", e);
            error!("{error_msg}");
            self.complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                .await;

            let _ = self
                .command
                .update_status(&UpdateWithdrawStatus {
                    withdraw_id: updated_withdraw.withdraw_id,
                    status: "failed".to_string(),
                })
                .await
                .map_err(|e2| {
                    error!("error {e2:?}");
                    e2
                });

            return Err(ServiceError::Custom(
                "failed to update withdraw status".into(),
            ));
        }

        let cache_keys: Vec<String> = vec![
            format!("withdraw:find_by_id:{}", withdraw_id),
            format!("withdraw:find_by_card:{}", req.card_number),
            format!("saldo:find_by_card:{}", req.card_number),
            "withdraw:find_all:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
            info!("Invalidated cache key: {}", key);
        }

        info!("success withdraw {:?}", updated_withdraw.withdraw_id);

        self.complete_tracing_success(&tracing_ctx, method, "Withdraw updated successfully")
            .await;

        Ok(ApiResponse {
            status: "success".into(),
            message: "updated withdraw successfully".into(),
            data: WithdrawResponse::from(updated_withdraw),
        })
    }

    async fn trashed_withdraw(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing withdraw id={withdraw_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "trash_withdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("withdraw_id", withdraw_id.to_string()),
            ],
        );

        let mut request = Request::new(withdraw_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.trashed(withdraw_id).await {
            Ok(withdraw) => {
                let response = WithdrawResponseDeleteAt::from(withdraw);
                info!("✅ Withdraw trashed successfully: id={withdraw_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Withdraw trashed successfully",
                )
                .await;

                let cache_keys = vec![
                    "withdraw:find_trashed:*",
                    "withdraw:find_active:*",
                    "withdraw:find_all:*",
                    "withdraw:find_by_id:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Withdraw trashed successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("❌ Failed to trash withdraw id={withdraw_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to trash withdraw: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(format!(
                    "Failed to trash withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn restore(
        &self,
        withdraw_id: i32,
    ) -> Result<ApiResponse<WithdrawResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring withdraw id={withdraw_id}");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_withdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("withdraw_id", withdraw_id.to_string()),
            ],
        );

        let mut request = Request::new(withdraw_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore(withdraw_id).await {
            Ok(withdraw) => {
                let response = WithdrawResponseDeleteAt::from(withdraw);
                info!("✅ Withdraw restored successfully: id={withdraw_id}");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "Withdraw restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "withdraw:find_trashed:*",
                    "withdraw:find_active:*",
                    "withdraw:find_all:*",
                    "withdraw:find_by_id:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Withdraw restored successfully".into(),
                    data: response,
                })
            }
            Err(e) => {
                error!("❌ Failed to restore withdraw id={withdraw_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore withdraw: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(format!(
                    "Failed to restore withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn delete_permanent(&self, withdraw_id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting withdraw id={withdraw_id}");

        let method = Method::Delete;
        let tracing_ctx = self.start_tracing(
            "delete_permanent_withdraw",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "delete_permanent"),
                KeyValue::new("withdraw_id", withdraw_id.to_string()),
            ],
        );

        let mut request = Request::new(withdraw_id);
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(withdraw_id).await {
            Ok(_) => {
                info!("✅ Withdraw permanently deleted: id={withdraw_id}");
                self.complete_tracing_success(&tracing_ctx, method, "Withdraw permanently deleted")
                    .await;

                let cache_keys = vec![
                    "withdraw:find_trashed:*",
                    "withdraw:find_active:*",
                    "withdraw:find_all:*",
                    "withdraw:find_by_id:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Withdraw permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to permanently delete withdraw id={withdraw_id}: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete withdraw: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(format!(
                    "Failed to permanently delete withdraw with id {withdraw_id}",
                )))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring all trashed withdraws");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "restore_all_withdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All withdraws restored successfully");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All withdraws restored successfully",
                )
                .await;

                let cache_keys = vec![
                    "withdraw:find_trashed:*",
                    "withdraw:find_active:*",
                    "withdraw:find_all:*",
                    "withdraw:find_by_id:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All withdraws restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to restore all withdraws: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to restore all withdraws: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to restore all trashed withdraws".to_string(),
                ))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting all trashed withdraws");

        let method = Method::Post;
        let tracing_ctx = self.start_tracing(
            "delete_all_withdraws",
            vec![
                KeyValue::new("component", "withdraw"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All withdraws permanently deleted");
                self.complete_tracing_success(
                    &tracing_ctx,
                    method,
                    "All withdraws permanently deleted",
                )
                .await;

                let cache_keys = vec![
                    "withdraw:find_trashed:*",
                    "withdraw:find_active:*",
                    "withdraw:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All withdraws permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to permanently delete all withdraws: {e:?}");
                self.complete_tracing_error(
                    &tracing_ctx,
                    method.clone(),
                    &format!("Failed to permanently delete all withdraws: {:?}", e),
                )
                .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete all trashed withdraws".to_string(),
                ))
            }
        }
    }
}
