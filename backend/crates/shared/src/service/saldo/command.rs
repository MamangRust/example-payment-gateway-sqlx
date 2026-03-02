use crate::{
    abstract_trait::{
        card::repository::query::DynCardQueryRepository,
        saldo::{
            repository::command::DynSaldoCommandRepository,
            service::command::SaldoCommandServiceTrait,
        },
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
        responses::{ApiResponse, SaldoResponse, SaldoResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
    utils::mask_card_number,
};
use anyhow::Result;
use async_trait::async_trait;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct SaldoCommandService {
    pub command: DynSaldoCommandRepository,
    pub card_query: DynCardQueryRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

pub struct SaldoCommandServiceDeps {
    pub card_query: DynCardQueryRepository,
    pub command: DynSaldoCommandRepository,
}

impl SaldoCommandService {
    pub fn new(deps: SaldoCommandServiceDeps, shared: &SharedResources) -> Result<Self> {
        let SaldoCommandServiceDeps {
            card_query,
            command,
        } = deps;

        Ok(Self {
            card_query,
            command,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl SaldoCommandServiceTrait for SaldoCommandService {
    async fn create(
        &self,
        request: &CreateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        if let Err(validation_errors) = request.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let masked_card = mask_card_number(&request.card_number);
        info!("Creating saldo for card_number={}", masked_card);

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "create_saldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "create"),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut request_obj = Request::new(request.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let _card = match self.card_query.find_by_card(&request.card_number).await {
            Ok(card) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method.clone(),
                        "Card fetched successfully",
                    )
                    .await;

                card
            }
            Err(e) => {
                let error_msg = format!("Failed to find card {}: {:?}", masked_card, e);

                error!("Failed to find card {}: {e:?}", masked_card);

                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

                return Err(ServiceError::Custom("Card not found".into()));
            }
        };

        let saldo = match self.command.create(request).await {
            Ok(saldo) => {
                info!(
                    "Saldo created successfully with id={} for card={}",
                    saldo.saldo_id, masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Saldo created successfully")
                    .await;
                saldo
            }
            Err(e) => {
                let error_msg = format!("Failed to create saldo for card {}: {:?}", masked_card, e);
                error!("Failed to create saldo for card {}: {e:?}", masked_card);
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom("Failed to create saldo".into()));
            }
        };

        let response = SaldoResponse::from(saldo);

        let cache_keys = vec![
            format!("saldo:find_by_card:card_number:{}", masked_card),
            format!("saldo:find_by_id:id:{}", response.clone().id),
            "saldo:find_all:*".to_string(),
            "saldo:find_by_active:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo created successfully".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        request: &UpdateSaldoRequest,
    ) -> Result<ApiResponse<SaldoResponse>, ServiceError> {
        if let Err(validation_errors) = request.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let saldo_id = request
            .saldo_id
            .ok_or_else(|| ServiceError::Custom("saldo_id is required".into()))?;

        let masked_card = mask_card_number(&request.card_number);
        info!("Updating saldo id={saldo_id} for card={}", masked_card);

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "update_saldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "update"),
                KeyValue::new("saldo_id", saldo_id.to_string()),
                KeyValue::new("card_number", masked_card.clone()),
            ],
        );

        let mut request_obj = Request::new(request.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request_obj);

        let _card = match self.card_query.find_by_card(&request.card_number).await {
            Ok(card) => {
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method.clone(),
                        "Card fetched successfully",
                    )
                    .await;

                card
            }
            Err(e) => {
                let error_msg = format!("Failed to find card {}: {:?}", masked_card, e);

                error!("Failed to find card {}: {e:?}", masked_card);

                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;

                return Err(ServiceError::Custom("Card not found".into()));
            }
        };

        let updated_saldo = match self.command.update(request).await {
            Ok(saldo) => {
                info!(
                    "Saldo updated successfully with id={} for card={}",
                    saldo.saldo_id, masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Saldo updated successfully")
                    .await;
                saldo
            }
            Err(e) => {
                let error_msg = format!(
                    "Failed to update saldo id={saldo_id} for card {}: {:?}",
                    masked_card, e
                );
                error!(
                    "Failed to update saldo id={saldo_id} for card {}: {e:?}",
                    masked_card
                );
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom("Failed to update saldo".into()));
            }
        };

        let response = SaldoResponse::from(updated_saldo);

        let cache_keys = vec![
            format!("saldo:find_by_id:id:{}", saldo_id),
            format!("saldo:find_by_card:card_number:{}", masked_card),
            "saldo:find_all:*".to_string(),
            "saldo:find_by_active:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo updated successfully".into(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing saldo with id={id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "trash_saldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let saldo = match self.command.trash(id).await {
            Ok(saldo) => {
                info!("Saldo trashed successfully with id={}", saldo.saldo_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Saldo trashed successfully")
                    .await;
                saldo
            }
            Err(e) => {
                error!("❌ Failed to trash saldo with id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to trash saldo: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom("Failed to trash saldo".into()));
            }
        };

        let response = SaldoResponseDeleteAt::from(saldo);

        let cache_keys = vec![
            format!("saldo:find_by_id:id:{}", id),
            format!("saldo:find_by_card:card_number:{}", response.card_number),
            "saldo:find_all:*".to_string(),
            "saldo:find_by_active:*".to_string(),
            "saldo:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo trashed successfully".into(),
            data: response,
        })
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<SaldoResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring saldo with id={id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "restore_saldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let saldo = match self.command.restore(id).await {
            Ok(saldo) => {
                info!("Saldo restored successfully with id={}", saldo.saldo_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Saldo restored successfully")
                    .await;
                saldo
            }
            Err(e) => {
                error!("❌ Failed to restore saldo with id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to restore saldo: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom("Failed to restore saldo".into()));
            }
        };

        let response = SaldoResponseDeleteAt::from(saldo);

        let cache_keys = vec![
            format!("saldo:find_by_id:id:{}", id),
            format!("saldo:find_by_card:card_number:{}", response.card_number),
            "saldo:find_all:*".to_string(),
            "saldo:find_by_active:*".to_string(),
            "saldo:find_by_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "Saldo restored successfully".into(),
            data: response,
        })
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💀 Permanently deleting saldo with id={id}");

        let method = Method::Delete;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "delete_saldo",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(id).await {
            Ok(_) => {
                info!("Saldo permanently deleted with id={id}");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Saldo permanently deleted successfully",
                    )
                    .await;

                let cache_keys = vec![
                    format!("saldo:find_by_id:id:{id}"),
                    "saldo:find_by_trashed:*".to_string(),
                    "saldo:find_by_active:*".to_string(),
                    "saldo:find_all:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "Saldo permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to permanently delete saldo with id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to permanently delete saldo: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete saldo".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("♻️ Restoring all trashed saldos");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "restore_all_saldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("All saldos restored successfully");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "All saldos restored successfully",
                    )
                    .await;

                let cache_keys = vec![
                    "saldo:find_by_trashed:*",
                    "saldo:find_by_active:*",
                    "saldo:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All saldos restored successfully".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to restore all saldos: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to restore all saldos: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom("Failed to restore all saldos".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💀 Permanently deleting all trashed saldos");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "delete_all_saldos",
            vec![
                KeyValue::new("component", "saldo"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("All saldos permanently deleted");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "All saldos permanently deleted successfully",
                    )
                    .await;

                let cache_keys = vec![
                    "saldo:find_by_trashed:*",
                    "saldo:find_by_active:*",
                    "saldo:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "All saldos permanently deleted".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("❌ Failed to delete all saldos: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to delete all saldos: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom("Failed to delete all saldos".into()))
            }
        }
    }
}
