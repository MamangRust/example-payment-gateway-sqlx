use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletePermanentSaldo {
    pub id: u32,
}
