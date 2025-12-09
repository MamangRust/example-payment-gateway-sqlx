mod command;
mod query;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::RoleCommandGrpcClientTrait;
pub use self::query::RoleQueryGrpcClientTrait;

#[async_trait]
pub trait RoleGrpcClientServiceTrait:
    RoleQueryGrpcClientTrait + RoleCommandGrpcClientTrait
{
}

pub type DynRoleGrpcClientService = Arc<dyn RoleGrpcClientServiceTrait + Send + Sync>;
