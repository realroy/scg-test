use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DOSCG {
    pub message: String,
}