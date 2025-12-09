use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        hashing::DynHashing,
        role::repository::query::DynRoleQueryRepository,
        user::{
            repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
            service::{command::DynUserCommandService, query::DynUserQueryService},
        },
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, Hashing, RedisPool},
    repository::{
        role::query::RoleQueryRepository,
        user::{command::UserCommandRepository, query::UserQueryRepository},
        user_role::UserRoleRepository,
    },
    service::user::{
        command::{UserCommandService, UserCommandServiceDeps},
        query::UserQueryService,
    },
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub user_command: DynUserCommandService,
    pub user_query: DynUserQueryService,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("user_command_service", &"DynUserCommandService")
            .field("user_query_service", &"DynUserQueryService")
            .finish()
    }
}

impl DependenciesInject {
    pub fn new(db: ConnectionPool, redis: RedisPool) -> Result<Self> {
        let cache = Arc::new(CacheStore::new(redis.pool.clone()));
        let hashing = Arc::new(Hashing::new()) as DynHashing;

        let user_query_repo =
            Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;
        let role_query_repo =
            Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;
        let user_role_repo =
            Arc::new(UserRoleRepository::new(db.clone())) as DynUserRoleCommandRepository;
        let user_command_repo =
            Arc::new(UserCommandRepository::new(db.clone())) as DynUserCommandRepository;

        let user_command_service_deps = UserCommandServiceDeps {
            query: user_query_repo.clone(),
            command: user_command_repo.clone(),
            hashing: hashing.clone(),
            user_role: user_role_repo.clone(),
            role: role_query_repo.clone(),
            cache_store: cache.clone(),
        };
        let user_command = Arc::new(
            UserCommandService::new(user_command_service_deps)
                .context("failed to initialize user command service")?,
        ) as DynUserCommandService;

        let user_query = Arc::new(
            UserQueryService::new(user_query_repo.clone(), cache.clone())
                .context("failed to initialize user query service")?,
        ) as DynUserQueryService;

        Ok(Self {
            user_command,
            user_query,
        })
    }
}
