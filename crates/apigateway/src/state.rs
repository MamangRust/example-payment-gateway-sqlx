use crate::{di::DependenciesInject, service::GrpcClients};
use anyhow::{Context, Result};
use shared::{
    abstract_trait::jwt::DynJwtService,
    config::{GrpcClientConfig, JwtConfig},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_config: DynJwtService,
    pub di_container: DependenciesInject,
}

impl AppState {
    pub async fn new(jwt_secret: &str) -> Result<Self> {
        let jwt_config = Arc::new(JwtConfig::new(jwt_secret)) as DynJwtService;
        let grpc_config = GrpcClientConfig::init().context("failed config grpc")?;

        let clients = GrpcClients::init(grpc_config)
            .await
            .context("failed grpc client")?;

        let di_container = {
            DependenciesInject::new(clients)
                .await
                .context("Failed to initialized depencency injection container")?
        };

        Ok(Self {
            jwt_config,
            di_container,
        })
    }
}

// trait MetricsRegister {
//     fn register_metrics(&mut self, metrics: &SystemMetrics);
// }

// impl MetricsRegister for Registry {
//     fn register_metrics(&mut self, metrics: &SystemMetrics) {
//         metrics.register(self);
//     }
// }
