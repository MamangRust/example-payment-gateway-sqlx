use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePermanentTopup {
    pub id: u32,
}
