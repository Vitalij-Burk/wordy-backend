use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::types::ID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: ID,
    pub jti: ID,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(sub: ID, jti: ID, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        Self { sub, jti, iat, exp }
    }
}
