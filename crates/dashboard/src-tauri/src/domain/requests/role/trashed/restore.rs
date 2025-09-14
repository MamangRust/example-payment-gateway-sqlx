use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestoreRoleTrashed {
    pub id: i32,
}
