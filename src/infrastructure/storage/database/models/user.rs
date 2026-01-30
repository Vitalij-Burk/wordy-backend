use crate::domain::models::user::User;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i32,

    pub key: String,
    pub name: String,
}

impl From<User> for UserEntity {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            key: value.key,
            name: value.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateDBUser {
    pub key: Option<String>,
    pub name: Option<String>,
}
