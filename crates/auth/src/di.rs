use anyhow::Result;
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
    config::ConnectionPool,
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
}

impl DependenciesInject {
    pub async fn new(deps: DependenciesInjectDeps) -> Result<Self> {
        let DependenciesInjectDeps {
            pool,
            hash,
            jwt_config,
        } = deps;

        let user_role =
            Arc::new(UserRoleRepository::new(pool.clone())) as DynUserRoleCommandRepository;

        let user_query = Arc::new(UserQueryRepository::new(pool.clone())) as DynUserQueryRepository;
        let user_command =
            Arc::new(UserCommandRepository::new(pool.clone())) as DynUserCommandRepository;

        let role = Arc::new(RoleQueryRepository::new(pool.clone())) as DynRoleQueryRepository;

        let refresh_command = Arc::new(RefreshTokenCommandRepository::new(pool.clone()))
            as DynRefreshTokenCommandRepository;

        let token_service = Arc::new(TokenService::new(
            jwt_config.clone(),
            refresh_command.clone(),
        )) as DynTokenService;

        let deps = AuthServiceDeps {
            query: user_query.clone(),
            command: user_command.clone(),
            jwt_config: jwt_config.clone(),
            hashing: hash.clone(),
            role: role.clone(),
            user_role: user_role.clone(),
            token: token_service.clone(),
            refresh_command: refresh_command.clone(),
        };

        let auth_service = Arc::new(AuthService::new(deps).await);

        Ok(Self { auth_service })
    }
}
