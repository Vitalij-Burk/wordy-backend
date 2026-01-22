#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i32,

    pub key: String,
    pub name: String,
}
