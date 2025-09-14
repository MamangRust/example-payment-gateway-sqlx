use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrashedWithdraw {
    pub id: i32,
}
