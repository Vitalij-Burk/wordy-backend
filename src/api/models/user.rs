use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDTO {
    pub id: i32,

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
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct _UpdateUserDTO {
    #[validate(length(min = 3, max = 30))]
    pub key: Option<String>,
    #[validate(length(min = 2, max = 20))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum _GetUserDTO {
    ById { id: i32 },
    ByKey { key: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum _DeleteUserDTO {
    ById { id: i32 },
    ByKey { key: String },
}
