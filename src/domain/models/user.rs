use heck::ToTitleCase;
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::models::user::CreateUserDTO;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,

    pub key: String,
    pub name: String,
}

impl User {
    pub fn new(key: &str, name: &str) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            key: key.to_string(),
            name: name.to_string().to_title_case(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserDomain {
    pub id: i32,

    pub key: String,
    pub name: String,
}

impl UserDomain {
    pub fn new(key: &str, name: &str) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            key: key.to_string(),
            name: name.to_string().to_title_case(),
        }
    }
}

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("Unknown error")]
    Unknown
}

impl TryFrom<CreateUserDTO> for UserDomain {
    type Error = UserDomainError;

    fn try_from(dto: CreateUserDTO) -> Result<Self, Self::Error> {
        Ok(UserDomain::new(&dto.key, &dto.name))
    }
}
