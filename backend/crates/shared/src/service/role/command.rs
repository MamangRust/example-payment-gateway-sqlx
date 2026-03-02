use crate::{
    abstract_trait::role::{
        repository::command::DynRoleCommandRepository, service::command::RoleCommandServiceTrait,
    },
    cache::CacheStore,
    context::shared_resources::SharedResources,
    domain::{
        requests::role::{CreateRoleRequest, UpdateRoleRequest},
        responses::{ApiResponse, RoleResponse, RoleResponseDeleteAt},
    },
    errors::{ServiceError, format_validation_errors},
    observability::{Method, TracingMetrics},
};
use anyhow::Result;
use async_trait::async_trait;
use opentelemetry::KeyValue;
use std::sync::Arc;
use tonic::Request;
use tracing::{error, info};
use validator::Validate;

pub struct RoleCommandService {
    pub command: DynRoleCommandRepository,
    pub tracing_metrics_core: TracingMetrics,
    pub cache_store: Arc<CacheStore>,
}

impl RoleCommandService {
    pub fn new(command: DynRoleCommandRepository, shared: &SharedResources) -> Result<Self> {
        Ok(Self {
            command,
            tracing_metrics_core: Arc::clone(&shared.tracing_metrics),
            cache_store: Arc::clone(&shared.cache_store),
        })
    }
}

#[async_trait]
impl RoleCommandServiceTrait for RoleCommandService {
    async fn create(
        &self,
        req: &CreateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        info!("🆕 Creating role with name: {}", req.name);

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "create_role",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "create"),
                KeyValue::new("role.name", req.name.clone()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let role = match self.command.create(req).await {
            Ok(role) => {
                info!("✅ Role created successfully with id={}", role.role_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Role created successfully")
                    .await;
                role
            }
            Err(e) => {
                let error_msg = format!("💥 Failed to create role with name {}: {e:?}", req.name);
                error!("{error_msg}");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg));
            }
        };

        let response = RoleResponse::from(role);

        let cache_keys = vec![
            "role:find_all:*",
            "role:find_by_active:*",
            "role_find_by_trashed",
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Role created successfully!".into(),
            data: response,
        })
    }

    async fn update(
        &self,
        req: &UpdateRoleRequest,
    ) -> Result<ApiResponse<RoleResponse>, ServiceError> {
        if let Err(validation_errors) = req.validate() {
            let error_msg = format_validation_errors(&validation_errors);
            error!("Validation failed: {error_msg}");
            return Err(ServiceError::Custom(error_msg));
        }

        let role_id = req
            .id
            .ok_or_else(|| ServiceError::Custom("role_id is required".into()))?;

        info!("🔄 Updating role id={role_id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "update_role",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "update"),
                KeyValue::new("role.id", role_id.to_string()),
            ],
        );

        let mut request = Request::new(req.clone());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let updated_role = match self.command.update(req).await {
            Ok(role) => {
                info!("✅ Role updated successfully with id={}", role.role_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Role updated successfully")
                    .await;
                role
            }
            Err(e) => {
                let error_msg = format!("💥 Failed to update role id={role_id}: {e:?}");
                error!("{error_msg}");
                self.tracing_metrics_core
                    .complete_tracing_error(&tracing_ctx, method.clone(), &error_msg)
                    .await;
                return Err(ServiceError::Custom(error_msg));
            }
        };

        let response = RoleResponse::from(updated_role);

        let cache_keys = vec![
            format!("role:find_by_id:id:{}", role_id),
            format!("role:find_by_name:name:{}", response.name),
            "role:find_all:*".to_string(),
            "role:find_active:*".to_string(),
            "role:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "✅ Role updated successfully!".into(),
            data: response,
        })
    }

    async fn trash(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError> {
        info!("🗑️ Trashing role id={id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "trash_role",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "trash"),
                KeyValue::new("role.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let role = match self.command.trash(id).await {
            Ok(role) => {
                info!("✅ Role trashed successfully with id={}", role.role_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Role trashed successfully")
                    .await;
                role
            }
            Err(e) => {
                error!("💥 Failed to trash role id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to trash role: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom("Failed to trash role".into()));
            }
        };

        let response = RoleResponseDeleteAt::from(role);

        let cache_keys = vec![
            format!("role:find_by_id:id:{}", id),
            format!("role:find_by_name:name:{}", response.name),
            "role:find_all:*".to_string(),
            "role:find_active:*".to_string(),
            "role:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "🗑️ Role trashed successfully!".into(),
            data: response,
        })
    }

    async fn restore(&self, id: i32) -> Result<ApiResponse<RoleResponseDeleteAt>, ServiceError> {
        info!("♻️ Restoring role id={id}");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "restore_role",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "restore"),
                KeyValue::new("role.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        let role = match self.command.restore(id).await {
            Ok(role) => {
                info!("✅ Role restored successfully with id={}", role.role_id);
                self.tracing_metrics_core
                    .complete_tracing_success(&tracing_ctx, method, "Role restored successfully")
                    .await;
                role
            }
            Err(e) => {
                error!("💥 Failed to restore role id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to restore role: {:?}", e),
                    )
                    .await;
                return Err(ServiceError::Custom("Failed to restore role".into()));
            }
        };

        let response = RoleResponseDeleteAt::from(role);

        let cache_keys = vec![
            format!("role:find_by_id:id:{}", id),
            format!("role:find_by_name:name:{}", response.name),
            "role:find_all:*".to_string(),
            "role:find_active:*".to_string(),
            "role:find_trashed:*".to_string(),
        ];

        for key in cache_keys {
            self.cache_store.delete_from_cache(&key).await;
        }

        Ok(ApiResponse {
            status: "success".into(),
            message: "♻️ Role restored successfully!".into(),
            data: response,
        })
    }

    async fn delete(&self, id: i32) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🧨 Permanently deleting role id={id}");

        let method = Method::Delete;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "delete_role",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "delete"),
                KeyValue::new("role.id", id.to_string()),
            ],
        );

        let mut request = Request::new(id);
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_permanent(id).await {
            Ok(_) => {
                info!("✅ Role permanently deleted with id={id}");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "Role permanently deleted successfully",
                    )
                    .await;

                let cache_keys = vec![
                    format!("role:find_by_id:id:{}", id),
                    "role:find_by_name:name:*".to_string(),
                    "role:find_all:*".to_string(),
                    "role:find_active:*".to_string(),
                    "role:find_trashed:*".to_string(),
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(&key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🧨 Role permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to permanently delete role id={id}: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to permanently delete role: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom(
                    "Failed to permanently delete role".into(),
                ))
            }
        }
    }

    async fn restore_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("🔄 Restoring ALL trashed roles");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "restore_all_roles",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "restore_all"),
            ],
        );

        let mut request = Request::new(());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.restore_all().await {
            Ok(_) => {
                info!("✅ All roles restored successfully");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "All roles restored successfully",
                    )
                    .await;

                let cache_keys = vec![
                    "role:find_trashed:*",
                    "role:find_active:*",
                    "role:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "🔄 All roles restored successfully!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to restore all roles: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to restore all roles: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom("Failed to restore all roles".into()))
            }
        }
    }

    async fn delete_all(&self) -> Result<ApiResponse<bool>, ServiceError> {
        info!("💣 Permanently deleting ALL trashed roles");

        let method = Method::Post;
        let tracing_ctx = self.tracing_metrics_core.start_tracing(
            "delete_all_roles",
            vec![
                KeyValue::new("component", "role"),
                KeyValue::new("operation", "delete_all"),
            ],
        );

        let mut request = Request::new(());
        self.tracing_metrics_core
            .inject_trace_context(&tracing_ctx.cx, &mut request);

        match self.command.delete_all().await {
            Ok(_) => {
                info!("✅ All roles permanently deleted");
                self.tracing_metrics_core
                    .complete_tracing_success(
                        &tracing_ctx,
                        method,
                        "All roles permanently deleted successfully",
                    )
                    .await;

                let cache_keys = vec![
                    "role:find_trashed:*",
                    "role:find_active:*",
                    "role:find_all:*",
                ];

                for key in cache_keys {
                    self.cache_store.delete_from_cache(key).await;
                }

                Ok(ApiResponse {
                    status: "success".into(),
                    message: "💣 All roles permanently deleted!".into(),
                    data: true,
                })
            }
            Err(e) => {
                error!("💥 Failed to delete all roles: {e:?}");
                self.tracing_metrics_core
                    .complete_tracing_error(
                        &tracing_ctx,
                        method.clone(),
                        &format!("Failed to delete all roles: {:?}", e),
                    )
                    .await;
                Err(ServiceError::Custom("Failed to delete all roles".into()))
            }
        }
    }
}
