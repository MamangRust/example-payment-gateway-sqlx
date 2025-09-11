use anyhow::Result;
use shared::{
    abstract_trait::role::{
        repository::{command::DynRoleCommandRepository, query::DynRoleQueryRepository},
        service::{command::DynRoleCommandService, query::DynRoleQueryService},
    },
    config::ConnectionPool,
    repository::role::{command::RoleCommandRepository, query::RoleQueryRepository},
    service::role::{command::RoleCommandService, query::RoleQueryService},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RoleQueryDeps {
    pub repo: DynRoleQueryRepository,
    pub service: DynRoleQueryService,
}

impl RoleQueryDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let repo = Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;
        let service = Arc::new(RoleQueryService::new(repo.clone()).await) as DynRoleQueryService;

        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct RoleCommandDeps {
    pub repo: DynRoleCommandRepository,
    pub service: DynRoleCommandService,
}

impl RoleCommandDeps {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let repo = Arc::new(RoleCommandRepository::new(db.clone())) as DynRoleCommandRepository;
        let service =
            Arc::new(RoleCommandService::new(repo.clone()).await) as DynRoleCommandService;

        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub role_query: RoleQueryDeps,
    pub role_command: RoleCommandDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let role_command = RoleCommandDeps::new(db.clone()).await?;
        let role_query = RoleQueryDeps::new(db.clone()).await?;

        Ok(Self {
            role_command,
            role_query,
        })
    }
}
