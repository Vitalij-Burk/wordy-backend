use crate::{
    domain::{
        models::user::User,
        traits::repositories::{repository::Repository, user_repository::IUserRepository},
    },
    infrastructure::storage::database::models::user::UserEntity,
};
use async_trait::async_trait;
use sqlx::{Error, postgres::PgPool};

#[derive(Clone)]
pub struct UserPostgresRepository {
    db: PgPool,
}

#[async_trait]
impl Repository for UserPostgresRepository {
    type Pool = PgPool;
    type Item = UserEntity;
    type Error = Error;

    fn new(db: PgPool) -> Self {
        Self { db: db }
    }

    async fn insert(&self, user: &UserEntity) -> Result<UserEntity, Error> {
        let db_user = sqlx::query_as!(
            UserEntity,
            "INSERT INTO users (id, key, name) VALUES ($1, $2, $3) RETURNING *",
            &user.id,
            &user.key,
            &user.name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }
}

#[async_trait]
impl IUserRepository for UserPostgresRepository {
    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(
            UserEntity,
            "SELECT id, key, name FROM users WHERE id=$1",
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }

    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(
            UserEntity,
            "SELECT id, key, name FROM users WHERE key=$1",
            key
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }
}
