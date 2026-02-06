use crate::{
    domain::{models::user::User, types::ID},
    infrastructure::utils::convert::{
        datetime_to_primitive::convert_datetime_utc_to_primitive,
        primitive_to_datetime::convert_primitive_to_datetime_utc,
    },
};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEntity {
    pub id: ID,
    pub hashed_password: String,

    pub key: String,
    pub name: String,

    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl From<&User> for UserEntity {
    fn from(value: &User) -> Self {
        let created_at = convert_datetime_utc_to_primitive(&value.created_at);
        let updated_at = convert_datetime_utc_to_primitive(&value.updated_at);

        Self {
            id: value.id,
            hashed_password: value.hashed_password.to_owned(),
            key: value.key.to_owned(),
            name: value.name.to_owned(),
            created_at: created_at,
            updated_at: updated_at,
        }
    }
}

impl From<&UserEntity> for User {
    fn from(value: &UserEntity) -> Self {
        let created_datetime = convert_primitive_to_datetime_utc(&value.created_at);
        let updated_datetime = convert_primitive_to_datetime_utc(&value.updated_at);

        Self {
            id: value.id,
            hashed_password: value.hashed_password.to_owned(),
            key: value.key.to_owned(),
            name: value.name.to_owned(),
            created_at: created_datetime,
            updated_at: updated_datetime,
        }
    }
}
