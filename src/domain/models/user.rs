use heck::ToTitleCase;
use rand::Rng;

use crate::infrastructure::storage::database::models::user::UserEntity;

#[derive(Debug, Clone)]
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

impl From<UserEntity> for User {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            key: value.key,
            name: value.name,
        }
    }
}
