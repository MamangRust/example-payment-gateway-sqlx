use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestoreUserTrashed {
    pub id: i32,
}
