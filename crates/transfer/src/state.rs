use crate::di::DependenciesInject;
use anyhow::{Context, Result};
use shared::config::ConnectionPool;

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
}

impl AppState {
    pub async fn new(pool: ConnectionPool) -> Result<Self> {
        let di_container = {
            DependenciesInject::new(pool.clone())
                .await
                .context("Failed to initialize dependency injection container")?
        };

        Ok(Self { di_container })
    }
}
