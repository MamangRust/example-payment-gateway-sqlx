mod command;
mod query;

pub use self::command::{DynRoleCommandService, RoleCommandServiceTrait};
pub use self::query::{DynRoleQueryService, RoleQueryServiceTrait};
