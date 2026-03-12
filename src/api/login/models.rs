use serde::{Deserialize, Serialize};

use crate::domain::types::ID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginByKey {
    pub key: String,
    pub plain_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimsDTO {
    pub sub: ID,
}
