use crate::di::{DependenciesInject, DependenciesInjectDeps};
use anyhow::{Context, Result};
use shared::{
    abstract_trait::{hashing::DynHashing, jwt::DynJwtService},
    config::{Config, ConnectionPool, Hashing, JwtConfig},
};

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
}

impl AppState {
    pub async fn new(pool: ConnectionPool, config: Config) -> Result<Self> {
        let jwt_config = Arc::new(JwtConfig::new(&config.jwt_secret)) as DynJwtService;
        let hashing = Arc::new(Hashing::new()) as DynHashing;

        let deps = DependenciesInjectDeps {
            pool: pool.clone(),
            hash: hashing,
            jwt_config,
        };

        let di_container = {
            DependenciesInject::new(deps)
                .await
                .context("Failed to initialize dependency injection container")?
        };

        Ok(Self { di_container })
    }
}
