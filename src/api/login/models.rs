use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginByKey {
    pub key: String,
    pub password: String,
}
