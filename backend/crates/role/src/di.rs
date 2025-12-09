use anyhow::{Context, Result};
use shared::{
    abstract_trait::role::{
        repository::{command::DynRoleCommandRepository, query::DynRoleQueryRepository},
        service::{command::DynRoleCommandService, query::DynRoleQueryService},
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    repository::role::{command::RoleCommandRepository, query::RoleQueryRepository},
    service::role::{command::RoleCommandService, query::RoleQueryService},
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub role_query: DynRoleQueryService,
    pub role_command: DynRoleCommandService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("role_query", &"DynRoleQueryService")
            .field("role_command", &"DynRoleCommandService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache_store = Arc::new(CacheStore::new(redis.pool.clone()));

        let role_query_repo =
            Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;
        let role_query = Arc::new(
            RoleQueryService::new(role_query_repo, cache_store.clone())
                .context("failed to initialize role query service")?,
        ) as DynRoleQueryService;

        let role_command_repo =
            Arc::new(RoleCommandRepository::new(db.clone())) as DynRoleCommandRepository;
        let role_command = Arc::new(
            RoleCommandService::new(role_command_repo, cache_store.clone())
                .context("failed to initialize role command service")?,
        ) as DynRoleCommandService;

        Ok(Self {
            role_query,
            role_command,
        })
    }
}
