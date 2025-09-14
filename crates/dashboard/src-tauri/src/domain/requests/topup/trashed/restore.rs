use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestoreTopupTrashed {
    pub id: u32,
}
