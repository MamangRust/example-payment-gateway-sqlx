use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindByIdTransfer {
    pub id: i32,
}
