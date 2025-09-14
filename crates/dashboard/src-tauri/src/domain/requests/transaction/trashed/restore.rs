use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestoreTransactionTrashed {
    pub id: u32,
}
