mod command;
mod query;

pub use self::command::{DynUserCommandService, UserCommandServiceTrait};
pub use self::query::{DynUserQueryService, UserQueryServiceTrait};
