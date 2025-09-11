use anyhow::Result;
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
    config::{ConnectionPool, Hashing},
    repository::{
        role::query::RoleQueryRepository,
        user::{command::UserCommandRepository, query::UserQueryRepository},
        user_role::UserRoleRepository,
    },
    service::user::{command::UserCommandService, query::UserQueryService},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserCommandDeps {
    pub repo: DynUserCommandRepository,
    pub service: DynUserCommandService,
}

impl UserCommandDeps {
    pub async fn new(
        db: ConnectionPool,
        user_query: DynUserQueryRepository,
        role_query: DynRoleQueryRepository,
        user_role: DynUserRoleCommandRepository,
        hashing: DynHashing,
    ) -> Result<Self> {
        let repo = Arc::new(UserCommandRepository::new(db.clone())) as DynUserCommandRepository;
        let service = Arc::new(
            UserCommandService::new(
                user_query.clone(),
                repo.clone(),
                hashing.clone(),
                user_role.clone(),
                role_query.clone(),
            )
            .await,
        ) as DynUserCommandService;

        Ok(Self { repo, service })
    }
}

#[derive(Clone)]
pub struct UserQueryDeps {
    pub service: DynUserQueryService,
}

impl UserQueryDeps {
    pub async fn new(repo: DynUserQueryRepository) -> Result<Self> {
        let service = Arc::new(UserQueryService::new(repo.clone()).await) as DynUserQueryService;

        Ok(Self { service })
    }
}

#[derive(Clone)]
pub struct DependenciesInject {
    pub user_command: UserCommandDeps,
    pub user_query: UserQueryDeps,
}

impl DependenciesInject {
    pub async fn new(db: ConnectionPool) -> Result<Self> {
        let query = Arc::new(UserQueryRepository::new(db.clone())) as DynUserQueryRepository;
        let role = Arc::new(RoleQueryRepository::new(db.clone())) as DynRoleQueryRepository;
        let user_role =
            Arc::new(UserRoleRepository::new(db.clone())) as DynUserRoleCommandRepository;

        let hashing = Arc::new(Hashing::new()) as DynHashing;

        let user_command = UserCommandDeps::new(
            db.clone(),
            query.clone(),
            role.clone(),
            user_role.clone(),
            hashing.clone(),
        )
        .await?;
        let user_query = UserQueryDeps::new(query.clone()).await?;

        Ok(Self {
            user_command,
            user_query,
        })
    }
}
