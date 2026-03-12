use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::{models::user::User, types::ID};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: ID,

    pub key: String,
    pub name: String,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            key: value.key,
            name: value.name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, max = 30))]
    pub key: String,
    #[validate(length(min = 2, max = 20))]
    pub name: String,
    #[validate(length(min = 6, max = 25))]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min = 3, max = 30))]
    pub key: Option<String>,
    #[validate(length(min = 2, max = 20))]
    pub name: Option<String>,
}
