use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllSaldoTrashed {
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}
