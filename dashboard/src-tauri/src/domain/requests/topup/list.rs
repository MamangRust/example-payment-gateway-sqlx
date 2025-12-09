use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllTopup {
    pub search: String,
    pub page: u32,
    pub page_size: u32,
}
