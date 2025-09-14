use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestoreSaldoTrashed {
    pub id: u32,
}
