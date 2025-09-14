use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePermanentTransfer {
    pub id: i32,
}
