use anyhow::{Context, Result};
use shared::{
    abstract_trait::{
        hashing::DynHashing,
        jwt::DynJwtService,
        refresh_token::command::DynRefreshTokenCommandRepository,
        role::repository::query::DynRoleQueryRepository,
        token::DynTokenService,
        user::repository::{command::DynUserCommandRepository, query::DynUserQueryRepository},
        user_roles::DynUserRoleCommandRepository,
    },
    cache::CacheStore,
    config::{ConnectionPool, RedisPool},
    repository::{
        refresh_token::RefreshTokenCommandRepository,
        role::query::RoleQueryRepository,
        user::{command::UserCommandRepository, query::UserQueryRepository},
        user_role::UserRoleRepository,
    },
    service::{
        auth::{AuthService, AuthServiceDeps},
        token::TokenService,
    },
};
use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct DependenciesInject {
    pub auth_service: Arc<AuthService>,
}

impl fmt::Debug for DependenciesInject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependenciesInject")
            .field("auth_service", &"<Arc<AuthService>>")
            .finish()
    }
}

#[derive(Clone)]
pub struct DependenciesInjectDeps {
    pub pool: ConnectionPool,
    pub hash: DynHashing,
    pub jwt_config: DynJwtService,
    pub redis: RedisPool,
}

impl DependenciesInject {
    pub fn new(deps: DependenciesInjectDeps) -> Result<Self> {
        let DependenciesInjectDeps {
            pool,
            hash,
            jwt_config,
            redis,
        } = deps;

        let user_role =
            Arc::new(UserRoleRepository::new(pool.clone())) as DynUserRoleCommandRepository;

        let user_query = Arc::new(UserQueryRepository::new(pool.clone())) as DynUserQueryRepository;
        let user_command =
            Arc::new(UserCommandRepository::new(pool.clone())) as DynUserCommandRepository;

        let role = Arc::new(RoleQueryRepository::new(pool.clone())) as DynRoleQueryRepository;

        let refresh_command = Arc::new(RefreshTokenCommandRepository::new(pool.clone()))
            as DynRefreshTokenCommandRepository;

        let cache = Arc::new(CacheStore::new(redis.pool.clone()));

        let token_service = Arc::new(TokenService::new(
            jwt_config.clone(),
            refresh_command.clone(),
        )) as DynTokenService;

        let deps = AuthServiceDeps {
            query: user_query,
            command: user_command,
            jwt_config: jwt_config.clone(),
            hashing: hash,
            role,
            user_role,
            token: token_service.clone(),
            refresh_command,
            cache_store: cache.clone(),
        };

        let auth_service =
            Arc::new(AuthService::new(deps).context("failed initialize auth service")?);

        Ok(Self { auth_service })
    }
}
