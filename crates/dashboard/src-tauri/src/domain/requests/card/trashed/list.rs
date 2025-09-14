use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindAllTrashedCard {
    pub page: u32,
    pub page_size: u32,
    pub search: String,
}
