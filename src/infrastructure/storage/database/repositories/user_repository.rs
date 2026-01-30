use crate::{
    domain::traits::repositories::{repository::Repository, user_repository::IUserRepository},
    infrastructure::storage::database::models::user::{UpdateDBUser, UserEntity},
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

    fn new(db: Self::Pool) -> Self {
        Self { db: db }
    }

    async fn insert(&self, user: &Self::Item) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(
            Self::Item,
            "INSERT INTO users (id, key, name) VALUES ($1, $2, $3) RETURNING *",
            &user.id,
            &user.key,
            &user.name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }

    async fn select_by_id(&self, id: &i32) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(
            Self::Item,
            "SELECT id, key, name FROM users WHERE id = $1",
            &id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }

    async fn delete_by_id(&self, id: &i32) -> Result<(), Self::Error> {
        sqlx::query_as!(Self::Item, "DELETE FROM users WHERE id = $1", &id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

#[async_trait]
impl IUserRepository for UserPostgresRepository {
    async fn select_by_key(&self, key: &str) -> Result<Self::Item, Self::Error> {
        let db_user = sqlx::query_as!(
            Self::Item,
            "SELECT id, key, name FROM users WHERE key=$1",
            key
        )
        .fetch_one(&self.db)
        .await?;

        Ok(db_user)
    }

    async fn update_by_id(
        &self,
        id: &i32,
        params: UpdateDBUser,
    ) -> Result<Self::Item, Self::Error> {
        let updated_db_user = sqlx::query_as!(
            Self::Item,
            "UPDATE users SET key = COALESCE($1, key), name = COALESCE($2, name) WHERE id = $3 RETURNING *",
            params.key,
            params.name,
            &id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(updated_db_user)
    }
}
